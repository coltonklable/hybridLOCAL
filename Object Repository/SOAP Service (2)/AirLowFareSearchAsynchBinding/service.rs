<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>7cd6f975-7127-48dc-8046-39ea69b79ffe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://localhost:8080/kestrel/AirService</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v49_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v49_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;air:LowFareSearchAsynchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; EnablePointToPointSearch=&quot;false&quot; EnablePointToPointAlternates=&quot;false&quot; MaxNumberOfExpertSolutions=&quot;0&quot; SolutionResult=&quot;false&quot; PreferCompleteItinerary=&quot;true&quot; MetaOptionIdentifier=&quot;re&quot; ReturnUpsellFare=&quot;false&quot; IncludeFareInfoMessages=&quot;false&quot; ReturnBrandedFares=&quot;true&quot; MultiGDSSearch=&quot;false&quot; ReturnMM=&quot;false&quot; CheckOBFees=&quot;fremunt celsa&quot; NSCC=&quot;fer&quot; FareInfoRules=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;sceptra tenens&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;turbi&quot; ProviderCode=&quot;flamm&quot; AgentID=&quot;profundum quippe ferant&quot;/>
         &lt;com:TerminalSessionInfo>nubibus ignem disiecitque&lt;/com:TerminalSessionInfo>
         &lt;com:NextResultReference ProviderCode=&quot;ac vi&quot;>rapidum iaculata e&lt;/com:NextResultReference>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:Airport Code=&quot;cae&quot;/>
               &lt;com:City Code=&quot;cir&quot;/>
               &lt;com:CityOrAirport Code=&quot;con&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;arce sce&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;ci&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:Airport Code=&quot;cer&quot;/>
               &lt;com:City Code=&quot;et &quot;/>
               &lt;com:CityOrAirport Code=&quot;fet&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;sed pate&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;ta&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime PreferredTime=&quot;austris aeoliam&quot;>
               &lt;com:TimeRange EarliestTime=&quot;sciret dare&quot; LatestTime=&quot;dare iussus&quot;/>
               &lt;com:SpecificTime Time=&quot;bella gero&quot;/>
               &lt;com:SearchExtraDays DaysBefore=&quot;3&quot; DaysAfter=&quot;3&quot;/>
            &lt;/air:SearchDepTime>
            &lt;air:SearchArvTime PreferredTime=&quot;dedit qui foedere&quot;>
               &lt;com:TimeRange EarliestTime=&quot;magno cum&quot; LatestTime=&quot;ac terras&quot;/>
               &lt;com:SpecificTime Time=&quot;regemque dedit&quot;/>
            &lt;/air:SearchArvTime>
            &lt;air:AirLegModifiers ProhibitOvernightLayovers=&quot;false&quot; MaxConnectionTime=&quot;100&quot; ReturnFirstAvailableOnly=&quot;true&quot; AllowDirectAccess=&quot;false&quot; ProhibitMultiAirportConnection=&quot;true&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;ArrivalTime&quot; MaxJourneyTime=&quot;99&quot;>
               &lt;air:PermittedCabins>
                  &lt;com:CabinClass Type=&quot;regina iovisque&quot;/>
               &lt;/air:PermittedCabins>
               &lt;air:PreferredCabins>
                  &lt;com:CabinClass Type=&quot;rapidum iaculata&quot;/>
               &lt;/air:PreferredCabins>
               &lt;air:PermittedCarriers>
                  &lt;com:Carrier Code=&quot;in&quot;/>
               &lt;/air:PermittedCarriers>
               &lt;air:ProhibitedCarriers>
                  &lt;com:Carrier Code=&quot;cu&quot;/>
               &lt;/air:ProhibitedCarriers>
               &lt;air:PreferredCarriers>
                  &lt;com:Carrier Code=&quot;in&quot;/>
               &lt;/air:PreferredCarriers>
               &lt;air:PermittedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;abd&quot;/>
                     &lt;com:City Code=&quot;con&quot;/>
                     &lt;com:CityOrAirport Code=&quot;ips&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PermittedConnectionPoints>
               &lt;air:ProhibitedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;fre&quot;/>
                     &lt;com:City Code=&quot;nim&quot;/>
                     &lt;com:CityOrAirport Code=&quot;mol&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:ProhibitedConnectionPoints>
               &lt;air:PreferredConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;mon&quot;/>
                     &lt;com:City Code=&quot;mol&quot;/>
                     &lt;com:CityOrAirport Code=&quot;ger&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PreferredConnectionPoints>
               &lt;air:PermittedBookingCodes>
                  &lt;air:BookingCode Code=&quot;ci&quot;/>
               &lt;/air:PermittedBookingCodes>
               &lt;air:PreferredBookingCodes>
                  &lt;air:BookingCode Code=&quot;mo&quot;/>
               &lt;/air:PreferredBookingCodes>
               &lt;air:PreferredAlliances>
                  &lt;air:Alliance Code=&quot;aris imponet&quot;/>
               &lt;/air:PreferredAlliances>
               &lt;air:ProhibitedBookingCodes>
                  &lt;air:BookingCode Code=&quot;ar&quot;/>
               &lt;/air:ProhibitedBookingCodes>
               &lt;air:DisfavoredAlliances>
                  &lt;air:Alliance Code=&quot;incedo regina&quot;/>
               &lt;/air:DisfavoredAlliances>
               &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;false&quot; StopDirects=&quot;true&quot; SingleOnlineCon=&quot;false&quot; DoubleOnlineCon=&quot;false&quot; TripleOnlineCon=&quot;false&quot; SingleInterlineCon=&quot;true&quot; DoubleInterlineCon=&quot;false&quot; TripleInterlineCon=&quot;true&quot;/>
               &lt;air:AnchorFlightData AirlineCode=&quot;pr&quot; FlightNumber=&quot;impos&quot; ConnectionIndicator=&quot;true&quot;/>
            &lt;/air:AirLegModifiers>
         &lt;/air:SearchAirLeg>
         &lt;air:SearchSpecificAirSegment DepartureTime=&quot;animos et temperat&quot; Carrier=&quot;so&quot; FlightNumber=&quot;patri&quot; Origin=&quot;mon&quot; Destination=&quot;lax&quot; SegmentIndex=&quot;3&quot;/>
         &lt;air:AirSearchModifiers DistanceType=&quot;MI&quot; IncludeFlightDetails=&quot;true&quot; AllowChangeOfAirport=&quot;true&quot; ProhibitOvernightLayovers=&quot;false&quot; MaxSolutions=&quot;100&quot; MaxConnectionTime=&quot;100&quot; SearchWeekends=&quot;true&quot; IncludeExtraSolutions=&quot;false&quot; ProhibitMultiAirportConnection=&quot;false&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;JourneyTime&quot; ExcludeOpenJawAirport=&quot;false&quot; ExcludeGroundTransportation=&quot;false&quot; MaxJourneyTime=&quot;99&quot; JetServiceOnly=&quot;true&quot;>
            &lt;air:DisfavoredProviders>
               &lt;com:Provider Code=&quot;coniu&quot;/>
            &lt;/air:DisfavoredProviders>
            &lt;air:PreferredProviders>
               &lt;com:Provider Code=&quot;rapid&quot;/>
            &lt;/air:PreferredProviders>
            &lt;air:DisfavoredCarriers>
               &lt;com:Carrier Code=&quot;tu&quot;/>
            &lt;/air:DisfavoredCarriers>
            &lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;au&quot;/>
            &lt;/air:PermittedCarriers>
            &lt;air:ProhibitedCarriers>
               &lt;com:Carrier Code=&quot;ae&quot;/>
            &lt;/air:ProhibitedCarriers>
            &lt;air:PreferredCarriers>
               &lt;com:Carrier Code=&quot;ae&quot;/>
            &lt;/air:PreferredCarriers>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;et coniunx&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;incedo regina iovisque&quot;/>
            &lt;/air:PreferredCabins>
            &lt;air:PreferredAlliances>
               &lt;air:Alliance Code=&quot;hic vasto rex&quot;/>
            &lt;/air:PreferredAlliances>
            &lt;air:DisfavoredAlliances>
               &lt;air:Alliance Code=&quot;ego quae&quot;/>
            &lt;/air:DisfavoredAlliances>
            &lt;air:PermittedBookingCodes>
               &lt;air:BookingCode Code=&quot;te&quot;/>
            &lt;/air:PermittedBookingCodes>
            &lt;air:PreferredBookingCodes>
               &lt;air:BookingCode Code=&quot;eg&quot;/>
            &lt;/air:PreferredBookingCodes>
            &lt;air:ProhibitedBookingCodes>
               &lt;air:BookingCode Code=&quot;ar&quot;/>
            &lt;/air:ProhibitedBookingCodes>
            &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;false&quot; StopDirects=&quot;true&quot; SingleOnlineCon=&quot;true&quot; DoubleOnlineCon=&quot;false&quot; TripleOnlineCon=&quot;false&quot; SingleInterlineCon=&quot;true&quot; DoubleInterlineCon=&quot;false&quot; TripleInterlineCon=&quot;false&quot;/>
            &lt;air:MaxLayoverDuration Domestic=&quot;100&quot; Gateway=&quot;100&quot; International=&quot;100&quot;/>
            &lt;air:NativeSearchModifier ProviderCode=&quot;aeoli&quot;>hic vasto&lt;/air:NativeSearchModifier>
         &lt;/air:AirSearchModifiers>
         &lt;air:SplitTicketingSearch RoundTrip=&quot;100&quot;/>
         &lt;air:JourneyData>
            &lt;air:AirSegment Key=&quot;et laxas&quot; Status=&quot;carcere frenat&quot; Passive=&quot;false&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; OpenSegment=&quot;false&quot; Group=&quot;3&quot; Carrier=&quot;im&quot; CabinClass=&quot;numen iunonis&quot; FlightNumber=&quot;per a&quot; Origin=&quot;et &quot; Destination=&quot;tem&quot; DepartureTime=&quot;ego quae&quot; ArrivalTime=&quot;iaculata e nubibus&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; ProviderCode=&quot;omnip&quot; SupplierCode=&quot;preme&quot; ParticipantLevel=&quot;talia flammato&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;vasto rex&quot; AvailabilityDisplayType=&quot;profundum quippe&quot; ClassOfService=&quot;pr&quot; ETicketability=&quot;Required&quot; Equipment=&quot;arc&quot; MarriageGroup=&quot;3&quot; NumberOfStops=&quot;3&quot; Seamless=&quot;true&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;rapidum iaculata&quot; HostTokenRef=&quot;tempestatesque sonoras imperio&quot; ProviderReservationInfoRef=&quot;rapidi secum verrantque&quot; PassiveProviderReservationInfoRef=&quot;e nubibus&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;e&quot; APISRequirementsRef=&quot;murmure montis&quot; BlackListed=&quot;false&quot; OperationalStatus=&quot;sonoras imperio&quot; NumberInParty=&quot;99&quot; RailCoachNumber=&quot;soro&quot; BookingDate=&quot;2010-01-13&quot; FlownSegment=&quot;false&quot; ScheduleChange=&quot;false&quot; BrandIndicator=&quot;insuper altos&quot;>
               &lt;com:SegmentRemark Key=&quot;maria ac terras&quot;>rates evertitque&lt;/com:SegmentRemark>
               &lt;air:SponsoredFltInfo SponsoredLNB=&quot;200&quot; NeutralLNB=&quot;200&quot; FltKey=&quot;iovis&quot;/>
               &lt;air:CodeshareInfo OperatingCarrier=&quot;in&quot; OperatingFlightNumber=&quot;sonor&quot;>rapidum iaculata e&lt;/air:CodeshareInfo>
               &lt;air:AirAvailInfo ProviderCode=&quot;auras&quot; HostTokenRef=&quot;iovisque et&quot;>
                  &lt;air:BookingCodeInfo CabinClass=&quot;flammato secum dea&quot; BookingCounts=&quot;hic vasto rex&quot;/>
                  &lt;air:FareTokenInfo FareInfoRef=&quot;aeoliam venit&quot; HostTokenRef=&quot;altos imposuit regemque&quot;/>
               &lt;/air:AirAvailInfo>
               &lt;air:FlightDetails Key=&quot;profundum quippe&quot; Origin=&quot;pre&quot; Destination=&quot;luc&quot; DepartureTime=&quot;luctantis ventos tempestatesque&quot; ArrivalTime=&quot;nubibus ignem&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; Equipment=&quot;et &quot; OnTimePerformance=&quot;100&quot; OriginTerminal=&quot;annos bella gero&quot; DestinationTerminal=&quot;ignem disiecitque&quot; GroundTime=&quot;100&quot; AutomatedCheckin=&quot;false&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;IgnoreSegment&quot;>
                     &lt;air:FareNote Key=&quot;quisquam numen&quot; Precedence=&quot;100&quot; NoteName=&quot;ipsa iovis&quot; FareInfoMessageRef=&quot;adorat praeterea aut&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>animos et temperat&lt;/air:FareNote>
                  &lt;/air:Connection>
                  &lt;air:Meals>NoMealService&lt;/air:Meals>
                  &lt;air:InFlightServices>et premere et&lt;/air:InFlightServices>
               &lt;/air:FlightDetails>
               &lt;air:FlightDetailsRef Key=&quot;faciat maria&quot;/>
               &lt;air:AlternateLocationDistanceRef Key=&quot;transfixo pectore flammas&quot;/>
               &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;StopOver&quot;>
                  &lt;air:FareNote Key=&quot;imposuit regemque&quot; Precedence=&quot;100&quot; NoteName=&quot;iunonis adorat praeterea&quot; FareInfoMessageRef=&quot;sceptra tenens&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>ac terras caelumque&lt;/air:FareNote>
               &lt;/air:Connection>
               &lt;com:SellMessage>regina iovisque et&lt;/com:SellMessage>
               &lt;air:RailCoachDetails RailCoachNumber=&quot;sceptra tenens&quot; AvailableRailSeats=&quot;terras caelumque&quot; RailSeatMapAvailability=&quot;true&quot;/>
            &lt;/air:AirSegment>
         &lt;/air:JourneyData>
         &lt;com:SearchPassenger Code=&quot;et la&quot; Age=&quot;100&quot; DOB=&quot;2006-11-16&quot; Gender=&quot;au&quot; PricePTCOnly=&quot;false&quot; BookingTravelerRef=&quot;pater omnipotens&quot; AccompaniedPassenger=&quot;false&quot; ResidencyType=&quot;Employee&quot; Key=&quot;vasto rex aeolus&quot;>
            &lt;com:Name Prefix=&quot;sciret dare iussus&quot; First=&quot;secum verrantque&quot; Middle=&quot;et carcere&quot; Last=&quot;iaculata e&quot; Suffix=&quot;divum incedo regina&quot; TravelerProfileId=&quot;100&quot;/>
            &lt;com:LoyaltyCard Key=&quot;insuper altos&quot; SupplierCode=&quot;im&quot; AllianceLevel=&quot;hic vasto rex&quot; MembershipProgram=&quot;murmure montis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; CardNumber=&quot;tot annos bella&quot; Status=&quot;profundum quippe&quot; MembershipStatus=&quot;feta furentibus austris&quot; FreeText=&quot;murmure montis&quot; SupplierType=&quot;Other&quot; Level=&quot;mollitque animos&quot; PriorityCode=&quot;rates evertitque aequora&quot; VendorLocationRef=&quot;premit ac&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;true&quot; ReservationLevel=&quot;false&quot;>
                  &lt;com:OperatedBy>sonoras imperio&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;illi indignantes&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;ventos tempestatesque sonoras&quot; Code=&quot;molemque&quot; Description=&quot;mollitque animos et&quot; Number=&quot;tenens mollitque&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:PersonalGeography>
               &lt;com:CountryCode>ci&lt;/com:CountryCode>
               &lt;com:StateProvinceCode>acuto&lt;/com:StateProvinceCode>
               &lt;com:CityCode>hon&lt;/com:CityCode>
            &lt;/com:PersonalGeography>
         &lt;/com:SearchPassenger>
         &lt;air:AirPricingModifiers ProhibitMinStayFares=&quot;false&quot; ProhibitMaxStayFares=&quot;false&quot; CurrencyType=&quot;ac &quot; ProhibitAdvancePurchaseFares=&quot;false&quot; ProhibitNonRefundableFares=&quot;false&quot; ProhibitRestrictedFares=&quot;false&quot; FaresIndicator=&quot;PublicFaresOnly&quot; FiledCurrency=&quot;aeo&quot; PlatingCarrier=&quot;pr&quot; OverrideCarrier=&quot;ve&quot; ETicketability=&quot;No&quot; AccountCodeFaresOnly=&quot;true&quot; Key=&quot;dare iussus&quot; ProhibitNonExchangeableFares=&quot;false&quot; ForceSegmentSelect=&quot;false&quot; InventoryRequestType=&quot;DirectAccess&quot; OneWayShop=&quot;false&quot; ProhibitUnbundledFareTypes=&quot;false&quot; ReturnServices=&quot;true&quot; ChannelId=&quot;aeol&quot; ReturnFareAttributes=&quot;false&quot; SellCheck=&quot;false&quot; ReturnFailedSegments=&quot;false&quot;>
            &lt;air:ProhibitedRuleCategories>
               &lt;air:FareRuleCategory Category=&quot;50&quot;/>
            &lt;/air:ProhibitedRuleCategories>
            &lt;air:AccountCodes>
               &lt;com:AccountCode Code=&quot;indignantes magno&quot; ProviderCode=&quot;venti&quot; SupplierCode=&quot;carce&quot; Type=&quot;flammato secum&quot;/>
            &lt;/air:AccountCodes>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;habenas ipsa&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;caelumque profundum quippe&quot; CompanyName=&quot;tenens mollitque&quot; ProviderCode=&quot;rates&quot; SupplierCode=&quot;laxas&quot;/>
            &lt;/air:ContractCodes>
            &lt;air:ExemptTaxes AllTaxes=&quot;true&quot; TaxTerritory=&quot;in&quot; CompanyName=&quot;abdidit atris&quot;>
               &lt;air:CountryCode>ae&lt;/air:CountryCode>
               &lt;air:TaxCategory>exspirantem transfixo&lt;/air:TaxCategory>
            &lt;/air:ExemptTaxes>
            &lt;air:PenaltyFareInformation ProhibitPenaltyFares=&quot;false&quot;>
               &lt;air:PenaltyInfo PenaltyApplies=&quot;Anytime&quot; NoShow=&quot;true&quot;>
                  &lt;air:Amount>montis circum&lt;/air:Amount>
                  &lt;air:Percentage>ventos tempestatesque&lt;/air:Percentage>
               &lt;/air:PenaltyInfo>
            &lt;/air:PenaltyFareInformation>
            &lt;com:DiscountCard Key=&quot;et temperat&quot; Code=&quot;foedere &quot; Description=&quot;verrantque per auras&quot; Number=&quot;hic vasto rex&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
            &lt;air:PromoCodes>
               &lt;air:PromoCode Code=&quot;vinclis et&quot; ProviderCode=&quot;monti&quot; SupplierCode=&quot;acuto&quot;/>
            &lt;/air:PromoCodes>
            &lt;air:ManualFareAdjustment AppliedOn=&quot;Base&quot; AdjustmentType=&quot;Percentage&quot; Value=&quot;1000.00&quot; PassengerRef=&quot;iras ni&quot; TicketDesignator=&quot;et quisquam&quot; FareType=&quot;carce&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;iras &quot; PseudoCityCode=&quot;iras ni&quot; Key=&quot;tempestatesque sonoras imperio&quot; IATA=&quot;austris &quot;/>
            &lt;air:BrandModifiers>
               &lt;air:FareFamilyDisplay ModifierType=&quot;divum incedo&quot;/>
               &lt;air:BasicDetailsOnly ReturnBasicDetails=&quot;true&quot;/>
            &lt;/air:BrandModifiers>
            &lt;air:MultiGDSSearchIndicator Type=&quot;montis circum&quot; ProviderCode=&quot;aeoli&quot; DefaultProvider=&quot;true&quot; PrivateFareCode=&quot;soror et&quot; PrivateFareCodeOnly=&quot;true&quot;/>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;aeoliam venit&quot;/>
            &lt;/air:PreferredCabins>
         &lt;/air:AirPricingModifiers>
         &lt;air:Enumeration>
            &lt;air:SolutionGroup Count=&quot;100&quot; TripType=&quot;BusinessOrFirst&quot; Diversification=&quot;FirstOD&quot; Tag=&quot;rapidi secum&quot; Primary=&quot;false&quot;>
               &lt;air:PermittedAccountCodes>
                  &lt;com:AccountCode Code=&quot;et carcere frenat&quot; ProviderCode=&quot;austr&quot; SupplierCode=&quot;claus&quot; Type=&quot;circum claustra&quot;/>
               &lt;/air:PermittedAccountCodes>
               &lt;air:PreferredAccountCodes>
                  &lt;com:AccountCode Code=&quot;magno cum&quot; ProviderCode=&quot;gero &quot; SupplierCode=&quot;rapid&quot; Type=&quot;et montis&quot;/>
               &lt;/air:PreferredAccountCodes>
               &lt;air:ProhibitedAccountCodes>
                  &lt;com:AccountCode Code=&quot;laxas sciret dare&quot; ProviderCode=&quot;auras&quot; SupplierCode=&quot;regin&quot; Type=&quot;pater omnipotens speluncis&quot;/>
               &lt;/air:ProhibitedAccountCodes>
               &lt;air:PermittedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;corri&quot; PseudoCityCode=&quot;patriam lo&quot; Key=&quot;infixit acuto&quot; IATA=&quot;frenat i&quot;/>
               &lt;/air:PermittedPointOfSales>
               &lt;air:ProhibitedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;monti&quot; PseudoCityCode=&quot;iaculata e&quot; Key=&quot;metuens molemque&quot; IATA=&quot;animos e&quot;/>
               &lt;/air:ProhibitedPointOfSales>
            &lt;/air:SolutionGroup>
         &lt;/air:Enumeration>
         &lt;air:AirExchangeModifiers BookingDate=&quot;dea corde&quot; TicketingDate=&quot;tempestatesque sonoras&quot; AccountCode=&quot;evertitque aequora ventis&quot; TicketDesignator=&quot;et quisquam&quot; AllowPenaltyFares=&quot;true&quot; PrivateFaresOnly=&quot;false&quot; UniversalRecordLocatorCode=&quot;flammas &quot; ProviderLocatorCode=&quot;transfix&quot; ProviderCode=&quot;cum gente&quot;>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;frenat illi&quot; CompanyName=&quot;premere et&quot; ProviderCode=&quot;iras &quot; SupplierCode=&quot;disie&quot;/>
            &lt;/air:ContractCodes>
         &lt;/air:AirExchangeModifiers>
         &lt;air:FlexExploreModifiers Type=&quot;Destination&quot; Radius=&quot;100&quot; GroupName=&quot;speluncis abdid&quot;>
            &lt;air:Destination>aeo&lt;/air:Destination>
         &lt;/air:FlexExploreModifiers>
         &lt;air:PCC>
            &lt;com:OverridePCC ProviderCode=&quot;imper&quot; PseudoCityCode=&quot;ni faciat&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;faciat mar&quot; Key=&quot;ignem disiecitque&quot; IATA=&quot;illum ex&quot;/>
            &lt;air:TicketAgency ProviderCode=&quot;anySimpleType&quot; PseudoCityCode=&quot;anySimpleType&quot;/>
         &lt;/air:PCC>
         &lt;air:FareRulesFilterCategory FareInfoRef=&quot;et premere&quot;>
            &lt;air:CategoryCode>volutans nimborum&lt;/air:CategoryCode>
         &lt;/air:FareRulesFilterCategory>
         &lt;com:FormOfPayment Key=&quot;feta furentibus&quot; Type=&quot;et temperat&quot; FulfillmentType=&quot;habenas ipsa&quot; FulfillmentLocation=&quot;iras ni&quot; FulfillmentIDType=&quot;Collection Reference&quot; FulfillmentIDNumber=&quot;maria ac terras&quot; IsAgentType=&quot;false&quot; AgentText=&quot;furentibus austris aeoliam&quot; ReuseFOP=&quot;corde volutans&quot; ExternalReference=&quot;disiecitque rates evertitque&quot; Reusable=&quot;false&quot; ProfileID=&quot;molemque et&quot; ProfileKey=&quot;aut supplex&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:CreditCard Type=&quot;fl&quot; Number=&quot;sceptra tenens mollitque&quot; ExpDate=&quot;2013-10&quot; Name=&quot;pater omnipotens&quot; CVV=&quot;foed&quot; ApprovalCode=&quot;flammas turbine&quot; ExtendedPayment=&quot;vasto rex&quot; CustomerReference=&quot;iussus habenas&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;circum claustra fremunt&quot; BankCountryCode=&quot;et&quot; BankStateCode=&quot;spelun&quot; Enett=&quot;false&quot; ProfileID=&quot;rapidi secum&quot; Key=&quot;disiecitque rates&quot;>
               &lt;com:PhoneNumber Key=&quot;dea corde volutans&quot; Type=&quot;Home&quot; Location=&quot;patriam lo&quot; CountryCode=&quot;celsa&quot; AreaCode=&quot;exspirante&quot; Number=&quot;coniunx una&quot; Extension=&quot;aequora ve&quot; Text=&quot;disiecitque rates&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;atris hoc&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;abdidit atris&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>et temperat iras&lt;/com:AddressName>
                  &lt;com:Street>ignem disiecitque rates&lt;/com:Street>
                  &lt;com:City>rex aeolus antro&lt;/com:City>
                  &lt;com:State>et montis&lt;/com:State>
                  &lt;com:PostalCode>imperio premit&lt;/com:PostalCode>
                  &lt;com:Country>se&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;regemque dedit&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;co&quot; Number=&quot;vinclis etvinclis et&quot; ExpDate=&quot;2001-01+05:30&quot; Name=&quot;ferant rapidi&quot; CVV=&quot;dare&quot; ApprovalCode=&quot;ventos tempestat&quot; IssueNumber=&quot;ni facia&quot; ProfileID=&quot;talia flammato secum&quot; Key=&quot;nimborum in&quot;>
               &lt;com:PhoneNumber Key=&quot;imperio premit&quot; Type=&quot;Email&quot; Location=&quot;caelumque &quot; CountryCode=&quot;aeolu&quot; AreaCode=&quot;illi indig&quot; Number=&quot;ac vinclis&quot; Extension=&quot;habenas ip&quot; Text=&quot;sed pater omnipotens&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;ni faciat&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;iussus habenas&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>dedit qui foedere&lt;/com:AddressName>
                  &lt;com:Street>ipsa iovis rapidum&lt;/com:Street>
                  &lt;com:City>patriam loca&lt;/com:City>
                  &lt;com:State>auras sed&lt;/com:State>
                  &lt;com:PostalCode>tenens mollitqu&lt;/com:PostalCode>
                  &lt;com:Country>ex&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;exspirantem transfixo&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;divum incedo regina&quot; Amount=&quot;furentibus austris&quot; DiscountAmount=&quot;volutans nimborum&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2014-07-25+05:30&quot; NotValidAfter=&quot;2005-10-21+05:30&quot;/>
            &lt;com:TicketNumber>gente tot ann&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;premere et&quot; RoutingNumber=&quot;et montis&quot; AccountNumber=&quot;ego quae&quot; CheckNumber=&quot;dea corde volutans&quot;/>
            &lt;com:Requisition Number=&quot;celsa sedet&quot; Category=&quot;Other&quot; Type=&quot;Cash&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;et&quot; CreditCardNumber=&quot;pater omnipotens&quot; ExpDate=&quot;2003-11+05:30&quot; Text=&quot;adorat praeterea aut&quot; Category=&quot;infixit acuto ast&quot; AcceptanceOverride=&quot;false&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;antro luctantis ventos&quot; AgencyBillingNumber=&quot;molemque et&quot; AgencyBillingPassword=&quot;altos imposuit&quot;/>
            &lt;com:UnitedNations Number=&quot;et coniunx una&quot;/>
            &lt;com:DirectPayment Text=&quot;in patriam loca&quot;/>
            &lt;com:AgentVoucher Number=&quot;quippe ferant&quot;/>
            &lt;com:PaymentAdvice Type=&quot;ni &quot; DocumentNumber=&quot;ventis illum&quot; IssueDate=&quot;2019-03-13+05:30&quot; IssueCity=&quot;iov&quot; OriginalFOP=&quot;imponet honorem&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;abdidit atris hoc&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;et laxas&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;illi indignantes magno&quot; BSPPassword=&quot;et premere&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;claustra fremunt&quot; ARCPassword=&quot;premit ac&quot;/>
         &lt;/com:FormOfPayment>
         &lt;air:AirSearchAsynchModifiers>
            &lt;air:InitialAsynchResult MaxWait=&quot;201&quot;/>
         &lt;/air:AirSearchAsynchModifiers>
      &lt;/air:LowFareSearchAsynchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/C:/Users/ramraja.sampathkumar/Desktop/SVT/POC_REST_SOAP/UAPI/wsdl/air_v49_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
