module CFMMRouter

using Documenter
using LinearAlgebra, SparseArrays, StaticArrays
using LBFGSB
using Printf
using JSON
using Base64

include("utils.jl")
include("cfmms.jl")
include("objectives.jl")
include("router.jl")


struct COp
    fptr::Ptr{Cvoid}
    n::Int
end

Base.eltype(A::COp) = Float64
Base.size(A::COp, d) = d <= 2 ? A.n : 1

const cfmmChar = Cuchar
const cfmmString = Ptr{cfmmChar}

function LinearAlgebra.mul!(y::StridedVector{Float64}, A::COp, x::StridedVector{Float64})
    @assert stride(x, 1) == 1
    @assert stride(y, 1) == 1
    i = ccall(A.fptr, Cint, (Ptr{Cdouble}, Ptr{Cdouble}), y, x)
    i == 0 || throw("Oh no")
    return y
end
function Base.:*(A::COp, x::AbstractVector{Float64})
    LinearAlgebra.mul!(similar(x), A, x)
end


Base.@ccallable function route(payload::Base.Cstring)::Base.Cstring
    try
        data = unsafe_string(payload)
        decoded = base64decode(data);
        if !isempty(data)  
            result = route_impl_2(decoded)
            result_ptr = pointer(result)
            result_c_str = Base.cconvert(Base.Cstring, result_ptr);
            return result_c_str
        end
    catch
        Base.invokelatest(Base.display_error, Base.catch_stack())
        return Base.display_error
    end
    return Base.Cstring("1")
end

function route_impl_2(data::Vector{UInt8})::String

    routes = Vector{Vector{Float64}}([])
    routes = JSON.parse(String(data), dicttype=Dict, inttype=BigInt)
    cfmms = Vector{ProductTwoCoin{Float64}}([])

    #Δin = Vector{Float64}([])
    Δin = [0, 0, 0]
    for cfmm in routes

        reverse!(cfmm)

        dest_coin = ""
        src_coin = ""
        fee = ""
        reserve2 = ""
        reserve1 = ""
    
        for i = 1:7
            if i == 1
                # Amt In
               # push!(parse(Float64, cfmm[i]), Δin)
            end
            if i == 2
                # Src Coin
                src_coin = parse(Float64,cfmm[i])
            end
            if i == 3
                # Dest Coin
                dest_coin = parse(Float64,cfmm[i])
            end
            if i == 4
                # Fee
                fee = 0.997
            end
            if i == 5
                # reserve1
                reserve1 = parse(Float64,cfmm[i])
            end
            if i == 6
                # reserve2
                reserve2 = parse(Float64,cfmm[i])
            end
            if i == 7
                # CFMM Model (ie ProductTwoCoin)
                type = parse(Float64,cfmm[i])
                if type == 0
                    push!(cfmms, ProductTwoCoin([reserve1, reserve2], fee, [src_coin, dest_coin]))
                end
                if type == 1
                    push!(cfmms, GeometricMeanTwoCoin([reserve1, reserve2], fee, [src_coin, dest_coin]))
                end
                if type == 2
                    push!(cfmms, GeometricMeanTwoCoin([reserve1, reserve2], fee, [src_coin, dest_coin]))
                end
                if type == 3
                    push!(cfmms, UniV3([reserve1, reserve2], fee, [src_coin, dest_coin]))
                end
            end
        end
    end

    ## We want to liquidate a basket of tokens 2 & 3 into token 1
    ## Build a routing problem with liquidation objective
    router = Router(
        BasketLiquidation(1, Δin),
        cfmms,
        maximum([maximum(cfmm.Ai) for cfmm in cfmms]),
    )

    ## Optimize!
    route!(router)

    ## Print results
    Ψ = round.(BigInt, netflows(router))

    #=
    We can also see the list of individual trades with each CFMM: 
    =#
    ## Print individual trades
    optimal_routes = Dict()
    if Ψ[1] > 0 
        for (i, (Δ, Λ)) in enumerate(zip(router.Δs, router.Λs))
            tokens = router.cfmms[i].Ai
            cfmm_basket = Dict()
            cfmm_tendered = Dict()
            for (ind, δ) in enumerate(Δ)
                if δ > eps()
                    cfmm_tendered[tokens[ind]] = string(round(BigInt, δ))
                end
            end
        
            cfmm_received = Dict()
            for (ind, λ) in enumerate(Λ)
                if λ > eps()
                    cfmm_received[tokens[ind]] = string(round(BigInt, λ))
                end 
            end
            cfmm_basket["tendered"] = cfmm_tendered
            cfmm_basket["received"] = cfmm_received
            optimal_routes[i] = cfmm_basket
        end
        data_dict = Dict()   
        data_dict["profit"] = Ψ[1]
        optimal_routes[0] = data_dict  
    end
    
    if  (haskey(optimal_routes,1) && haskey(optimal_routes[1]["tendered"],1)) || ( haskey(optimal_routes,3) && haskey(optimal_routes[3]["tendered"],1))
        println("Net trade: $Ψ")
        println("$(optimal_routes)") 
        return JSON.json(optimal_routes)
    else
        return JSON.json([])
    end
end


Base.@ccallable function julia_cfmmrouter()::Cint
    try
        return 11
    catch
        Base.invokelatest(Base.display_error, Base.catch_stack())
        return 1
    end
    return 0
end

end