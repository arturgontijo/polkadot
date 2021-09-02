initSidebarItems({"enum":[["Error","Errors associated with [`MatchesFungibles`] operation."]],"struct":[["Decoded","Implementation of `Convert<Vec<u8>, _>` using the parity scale codec."],["Encoded","Implementation of `Convert<_, Vec<u8>>` using the parity scale codec."],["Identity","Simple pass-through which implements `BytesConversion` while not doing any conversion."],["JustTry","Implementation of `Convert` trait using `TryFrom`."]],"trait":[["ClaimAssets","Define any handlers for the `AssetClaim` instruction."],["Convert","Generic third-party conversion trait. Use this when you don’t want to force the user to use default implementations of `From` and `Into` for the types you wish to convert between."],["ConvertOrigin","A converter `trait` for origin types."],["DropAssets","Define a handler for when some non-empty `Assets` value should be dropped."],["FilterAssetLocation","Filters assets/location pairs."],["InvertLocation","Means of inverting a location: given a location which describes a `target` interpreted from the `source`, this will provide the corresponding location which describes the `source`."],["MatchesFungible",""],["MatchesFungibles",""],["OnResponse","Define what needs to be done upon receiving a query response."],["ShouldExecute","Trait to determine whether the execution engine should actually execute a given XCM."],["TransactAsset","Facility for asset transacting."],["VersionChangeNotifier","Trait for a type which handles notifying a destination of XCM version changes."],["WeightBounds","Determine the weight of an XCM message."],["WeightTrader","Charge for weight in order to execute XCM."]]});