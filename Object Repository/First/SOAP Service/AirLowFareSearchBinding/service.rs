<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>94ac8134-7cf4-4a47-a5d0-bf4301adaff0</elementGuidId>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ses=&quot;http://www.travelport.com/soa/common/security/SessionContext_v1&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header>
      &lt;ses:SessionContext>
         &lt;ses:SessTok id=&quot;una cum gente&quot;/>
         &lt;ses:SessProp nm=&quot;sonoras imperio&quot; val=&quot;patriam loca feta&quot;/>
      &lt;/ses:SessionContext>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;air:LowFareSearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; EnablePointToPointSearch=&quot;false&quot; EnablePointToPointAlternates=&quot;false&quot; MaxNumberOfExpertSolutions=&quot;0&quot; SolutionResult=&quot;false&quot; PreferCompleteItinerary=&quot;true&quot; MetaOptionIdentifier=&quot;re&quot; ReturnUpsellFare=&quot;false&quot; IncludeFareInfoMessages=&quot;false&quot; ReturnBrandedFares=&quot;true&quot; MultiGDSSearch=&quot;false&quot; ReturnMM=&quot;false&quot; CheckOBFees=&quot;fremunt celsa&quot; NSCC=&quot;fer&quot; FareInfoRules=&quot;false&quot; MostRestrictivePenalties=&quot;false&quot; PolicyReference=&quot;sceptra tenens&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;turbine corripuit scopuloque&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;flamm&quot; ProviderCode=&quot;profu&quot; AgentID=&quot;nubibus ignem disiecitque&quot;/>
         &lt;com:TerminalSessionInfo>ac vinclis et&lt;/com:TerminalSessionInfo>
         &lt;com:NextResultReference ProviderCode=&quot;rapid&quot;>caelumque profundum&lt;/com:NextResultReference>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:Airport Code=&quot;cir&quot;/>
               &lt;com:City Code=&quot;con&quot;/>
               &lt;com:CityOrAirport Code=&quot;arc&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;circum c&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;ce&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:Airport Code=&quot;et &quot;/>
               &lt;com:City Code=&quot;fet&quot;/>
               &lt;com:CityOrAirport Code=&quot;sed&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;talia fl&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;au&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime PreferredTime=&quot;sciret dare&quot;>
               &lt;com:TimeRange EarliestTime=&quot;dare iussus&quot; LatestTime=&quot;bella gero&quot;/>
               &lt;com:SpecificTime Time=&quot;dedit qui foedere&quot;/>
               &lt;com:SearchExtraDays DaysBefore=&quot;3&quot; DaysAfter=&quot;3&quot;/>
            &lt;/air:SearchDepTime>
            &lt;air:SearchArvTime PreferredTime=&quot;magno cum&quot;>
               &lt;com:TimeRange EarliestTime=&quot;ac terras&quot; LatestTime=&quot;regemque dedit&quot;/>
               &lt;com:SpecificTime Time=&quot;infixit acuto&quot;/>
            &lt;/air:SearchArvTime>
            &lt;air:AirLegModifiers ProhibitOvernightLayovers=&quot;false&quot; MaxConnectionTime=&quot;100&quot; ReturnFirstAvailableOnly=&quot;true&quot; AllowDirectAccess=&quot;false&quot; ProhibitMultiAirportConnection=&quot;false&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;DepartureTime&quot; MaxJourneyTime=&quot;99&quot;>
               &lt;air:PermittedCabins>
                  &lt;com:CabinClass Type=&quot;rapidum iaculata&quot;/>
               &lt;/air:PermittedCabins>
               &lt;air:PreferredCabins>
                  &lt;com:CabinClass Type=&quot;insuper altos&quot;/>
               &lt;/air:PreferredCabins>
               &lt;air:PermittedCarriers>
                  &lt;com:Carrier Code=&quot;cu&quot;/>
               &lt;/air:PermittedCarriers>
               &lt;air:ProhibitedCarriers>
                  &lt;com:Carrier Code=&quot;in&quot;/>
               &lt;/air:ProhibitedCarriers>
               &lt;air:PreferredCarriers>
                  &lt;com:Carrier Code=&quot;ab&quot;/>
               &lt;/air:PreferredCarriers>
               &lt;air:PermittedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;con&quot;/>
                     &lt;com:City Code=&quot;ips&quot;/>
                     &lt;com:CityOrAirport Code=&quot;fre&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PermittedConnectionPoints>
               &lt;air:ProhibitedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;nim&quot;/>
                     &lt;com:City Code=&quot;mol&quot;/>
                     &lt;com:CityOrAirport Code=&quot;mon&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:ProhibitedConnectionPoints>
               &lt;air:PreferredConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;mol&quot;/>
                     &lt;com:City Code=&quot;ger&quot;/>
                     &lt;com:CityOrAirport Code=&quot;cir&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PreferredConnectionPoints>
               &lt;air:PermittedBookingCodes>
                  &lt;air:BookingCode Code=&quot;mo&quot;/>
               &lt;/air:PermittedBookingCodes>
               &lt;air:PreferredBookingCodes>
                  &lt;air:BookingCode Code=&quot;ar&quot;/>
               &lt;/air:PreferredBookingCodes>
               &lt;air:PreferredAlliances>
                  &lt;air:Alliance Code=&quot;aris imponet&quot;/>
               &lt;/air:PreferredAlliances>
               &lt;air:ProhibitedBookingCodes>
                  &lt;air:BookingCode Code=&quot;in&quot;/>
               &lt;/air:ProhibitedBookingCodes>
               &lt;air:DisfavoredAlliances>
                  &lt;air:Alliance Code=&quot;ac vinclis&quot;/>
               &lt;/air:DisfavoredAlliances>
               &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;false&quot; StopDirects=&quot;false&quot; SingleOnlineCon=&quot;false&quot; DoubleOnlineCon=&quot;true&quot; TripleOnlineCon=&quot;false&quot; SingleInterlineCon=&quot;true&quot; DoubleInterlineCon=&quot;false&quot; TripleInterlineCon=&quot;true&quot;/>
               &lt;air:AnchorFlightData AirlineCode=&quot;im&quot; FlightNumber=&quot;magno&quot; ConnectionIndicator=&quot;true&quot;/>
            &lt;/air:AirLegModifiers>
         &lt;/air:SearchAirLeg>
         &lt;air:SearchSpecificAirSegment DepartureTime=&quot;sonoras imperio&quot; Carrier=&quot;pa&quot; FlightNumber=&quot;monti&quot; Origin=&quot;lax&quot; Destination=&quot;e n&quot; SegmentIndex=&quot;3&quot;/>
         &lt;air:AirSearchModifiers DistanceType=&quot;MI&quot; IncludeFlightDetails=&quot;true&quot; AllowChangeOfAirport=&quot;true&quot; ProhibitOvernightLayovers=&quot;false&quot; MaxSolutions=&quot;100&quot; MaxConnectionTime=&quot;100&quot; SearchWeekends=&quot;false&quot; IncludeExtraSolutions=&quot;false&quot; ProhibitMultiAirportConnection=&quot;true&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;ArrivalTime&quot; ExcludeOpenJawAirport=&quot;false&quot; ExcludeGroundTransportation=&quot;false&quot; MaxJourneyTime=&quot;99&quot; JetServiceOnly=&quot;false&quot;>
            &lt;air:DisfavoredProviders>
               &lt;com:Provider Code=&quot;rapid&quot;/>
            &lt;/air:DisfavoredProviders>
            &lt;air:PreferredProviders>
               &lt;com:Provider Code=&quot;turbi&quot;/>
            &lt;/air:PreferredProviders>
            &lt;air:DisfavoredCarriers>
               &lt;com:Carrier Code=&quot;au&quot;/>
            &lt;/air:DisfavoredCarriers>
            &lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;ae&quot;/>
            &lt;/air:PermittedCarriers>
            &lt;air:ProhibitedCarriers>
               &lt;com:Carrier Code=&quot;ae&quot;/>
            &lt;/air:ProhibitedCarriers>
            &lt;air:PreferredCarriers>
               &lt;com:Carrier Code=&quot;et&quot;/>
            &lt;/air:PreferredCarriers>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;incedo regina iovisque&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;hic vasto rex&quot;/>
            &lt;/air:PreferredCabins>
            &lt;air:PreferredAlliances>
               &lt;air:Alliance Code=&quot;ego quae&quot;/>
            &lt;/air:PreferredAlliances>
            &lt;air:DisfavoredAlliances>
               &lt;air:Alliance Code=&quot;tempestatesque sonoras&quot;/>
            &lt;/air:DisfavoredAlliances>
            &lt;air:PermittedBookingCodes>
               &lt;air:BookingCode Code=&quot;eg&quot;/>
            &lt;/air:PermittedBookingCodes>
            &lt;air:PreferredBookingCodes>
               &lt;air:BookingCode Code=&quot;ar&quot;/>
            &lt;/air:PreferredBookingCodes>
            &lt;air:ProhibitedBookingCodes>
               &lt;air:BookingCode Code=&quot;mo&quot;/>
            &lt;/air:ProhibitedBookingCodes>
            &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;true&quot; StopDirects=&quot;false&quot; SingleOnlineCon=&quot;false&quot; DoubleOnlineCon=&quot;true&quot; TripleOnlineCon=&quot;false&quot; SingleInterlineCon=&quot;false&quot; DoubleInterlineCon=&quot;false&quot; TripleInterlineCon=&quot;true&quot;/>
            &lt;air:MaxLayoverDuration Domestic=&quot;100&quot; Gateway=&quot;100&quot; International=&quot;100&quot;/>
            &lt;air:NativeSearchModifier ProviderCode=&quot;hic v&quot;>et laxas&lt;/air:NativeSearchModifier>
         &lt;/air:AirSearchModifiers>
         &lt;air:SplitTicketingSearch RoundTrip=&quot;100&quot;/>
         &lt;air:JourneyData>
            &lt;air:AirSegment Key=&quot;carcere frenat&quot; Status=&quot;murmure montis circum&quot; Passive=&quot;false&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; OpenSegment=&quot;false&quot; Group=&quot;3&quot; Carrier=&quot;nu&quot; CabinClass=&quot;per auras&quot; FlightNumber=&quot;et ca&quot; Origin=&quot;tem&quot; Destination=&quot;ego&quot; DepartureTime=&quot;iaculata e nubibus&quot; ArrivalTime=&quot;omnipotens speluncis&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; ProviderCode=&quot;preme&quot; SupplierCode=&quot;talia&quot; ParticipantLevel=&quot;terras caelumque&quot; LinkAvailability=&quot;false&quot; PolledAvailabilityOption=&quot;profundum quippe&quot; AvailabilityDisplayType=&quot;premit ac&quot; ClassOfService=&quot;il&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;luc&quot; MarriageGroup=&quot;3&quot; NumberOfStops=&quot;3&quot; Seamless=&quot;false&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;tempestatesque sonoras imperio&quot; HostTokenRef=&quot;rapidi secum verrantque&quot; ProviderReservationInfoRef=&quot;e nubibus&quot; PassiveProviderReservationInfoRef=&quot;tenens mollitque&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;m&quot; APISRequirementsRef=&quot;premere et&quot; BlackListed=&quot;true&quot; OperationalStatus=&quot;soror et&quot; NumberInParty=&quot;99&quot; RailCoachNumber=&quot;impe&quot; BookingDate=&quot;2006-05-07&quot; FlownSegment=&quot;false&quot; ScheduleChange=&quot;false&quot; BrandIndicator=&quot;maria ac terras&quot;>
               &lt;com:SegmentRemark Key=&quot;rates evertitque&quot;>iovis rapidum&lt;/com:SegmentRemark>
               &lt;air:SponsoredFltInfo SponsoredLNB=&quot;200&quot; NeutralLNB=&quot;200&quot; FltKey=&quot;indig&quot;/>
               &lt;air:CodeshareInfo OperatingCarrier=&quot;so&quot; OperatingFlightNumber=&quot;rapid&quot;>auras sed&lt;/air:CodeshareInfo>
               &lt;air:AirAvailInfo ProviderCode=&quot;iovis&quot; HostTokenRef=&quot;flammato secum dea&quot;>
                  &lt;air:BookingCodeInfo CabinClass=&quot;hic vasto rex&quot; BookingCounts=&quot;aeoliam venit&quot;/>
                  &lt;air:FareTokenInfo FareInfoRef=&quot;altos imposuit regemque&quot; HostTokenRef=&quot;profundum quippe&quot;/>
               &lt;/air:AirAvailInfo>
               &lt;air:FlightDetails Key=&quot;premit ac vinclis&quot; Origin=&quot;luc&quot; Destination=&quot;luc&quot; DepartureTime=&quot;nubibus ignem&quot; ArrivalTime=&quot;et soror&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; Equipment=&quot;ann&quot; OnTimePerformance=&quot;100&quot; OriginTerminal=&quot;ignem disiecitque&quot; DestinationTerminal=&quot;iovisque et soror&quot; GroundTime=&quot;100&quot; AutomatedCheckin=&quot;false&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;NoStopOver&quot;>
                     &lt;air:FareNote Key=&quot;ipsa iovis&quot; Precedence=&quot;100&quot; NoteName=&quot;adorat praeterea aut&quot; FareInfoMessageRef=&quot;omnipotens speluncis abdidit&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>sciret dare&lt;/air:FareNote>
                  &lt;/air:Connection>
                  &lt;air:Meals>Breakfast&lt;/air:Meals>
                  &lt;air:InFlightServices>faciat maria&lt;/air:InFlightServices>
               &lt;/air:FlightDetails>
               &lt;air:FlightDetailsRef Key=&quot;transfixo pectore flammas&quot;/>
               &lt;air:AlternateLocationDistanceRef Key=&quot;claustra fremunt&quot;/>
               &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;NoStopOver&quot;>
                  &lt;air:FareNote Key=&quot;iunonis adorat praeterea&quot; Precedence=&quot;100&quot; NoteName=&quot;sceptra tenens&quot; FareInfoMessageRef=&quot;sciret dare iussus&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>regina iovisque et&lt;/air:FareNote>
               &lt;/air:Connection>
               &lt;com:SellMessage>sceptra tenens&lt;/com:SellMessage>
               &lt;air:RailCoachDetails RailCoachNumber=&quot;terras caelumque&quot; AvailableRailSeats=&quot;aut supplex aris&quot; RailSeatMapAvailability=&quot;false&quot;/>
            &lt;/air:AirSegment>
         &lt;/air:JourneyData>
         &lt;com:SearchPassenger Code=&quot;feran&quot; Age=&quot;100&quot; DOB=&quot;2006-01-09&quot; Gender=&quot;ip&quot; PricePTCOnly=&quot;false&quot; BookingTravelerRef=&quot;sedet aeolus&quot; AccompaniedPassenger=&quot;false&quot; ResidencyType=&quot;Resident&quot; Key=&quot;sciret dare iussus&quot;>
            &lt;com:Name Prefix=&quot;secum verrantque&quot; First=&quot;et carcere&quot; Middle=&quot;iaculata e&quot; Last=&quot;divum incedo regina&quot; Suffix=&quot;insuper altos&quot; TravelerProfileId=&quot;100&quot;/>
            &lt;com:LoyaltyCard Key=&quot;imperio premit&quot; SupplierCode=&quot;hi&quot; AllianceLevel=&quot;murmure montis&quot; MembershipProgram=&quot;illi indignantes magno&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; CardNumber=&quot;profundum quippe&quot; Status=&quot;feta furentibus austris&quot; MembershipStatus=&quot;murmure montis&quot; FreeText=&quot;numen iunonis adorat&quot; SupplierType=&quot;Other&quot; Level=&quot;rates evertitque aequora&quot; PriorityCode=&quot;premit ac&quot; VendorLocationRef=&quot;quippe ferant&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;true&quot; ReservationLevel=&quot;false&quot;>
                  &lt;com:OperatedBy>illi indignantes&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;ventos tempestatesque sonoras&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;molemque et montis&quot; Code=&quot;mollitqu&quot; Description=&quot;tenens mollitque&quot; Number=&quot;circum claustra&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:PersonalGeography>
               &lt;com:CountryCode>ac&lt;/com:CountryCode>
               &lt;com:StateProvinceCode>honore&lt;/com:StateProvinceCode>
               &lt;com:CityCode>ac&lt;/com:CityCode>
            &lt;/com:PersonalGeography>
         &lt;/com:SearchPassenger>
         &lt;air:AirPricingModifiers ProhibitMinStayFares=&quot;false&quot; ProhibitMaxStayFares=&quot;false&quot; CurrencyType=&quot;pre&quot; ProhibitAdvancePurchaseFares=&quot;false&quot; ProhibitNonRefundableFares=&quot;false&quot; ProhibitRestrictedFares=&quot;false&quot; FaresIndicator=&quot;PrivateFaresOnly&quot; FiledCurrency=&quot;pro&quot; PlatingCarrier=&quot;ve&quot; OverrideCarrier=&quot;ig&quot; ETicketability=&quot;No&quot; AccountCodeFaresOnly=&quot;true&quot; Key=&quot;supplex aris&quot; ProhibitNonExchangeableFares=&quot;false&quot; ForceSegmentSelect=&quot;false&quot; InventoryRequestType=&quot;DirectAccess&quot; OneWayShop=&quot;false&quot; ProhibitUnbundledFareTypes=&quot;true&quot; ReturnServices=&quot;false&quot; ChannelId=&quot;indi&quot; ReturnFareAttributes=&quot;false&quot; SellCheck=&quot;false&quot; ReturnFailedSegments=&quot;false&quot; SellCity=&quot;ven&quot; TicketingCity=&quot;car&quot;>
            &lt;air:ProhibitedRuleCategories>
               &lt;air:FareRuleCategory Category=&quot;50&quot;/>
            &lt;/air:ProhibitedRuleCategories>
            &lt;air:AccountCodes>
               &lt;com:AccountCode Code=&quot;flammato secum&quot; ProviderCode=&quot;haben&quot; SupplierCode=&quot;caelu&quot; Type=&quot;tenens mollitque&quot;/>
            &lt;/air:AccountCodes>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;rates evertitque aequora&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;laxas sciret&quot; CompanyName=&quot;gente tot&quot; ProviderCode=&quot;spelu&quot; SupplierCode=&quot;patri&quot;/>
            &lt;/air:ContractCodes>
            &lt;air:ExemptTaxes AllTaxes=&quot;false&quot; TaxTerritory=&quot;ex&quot; CompanyName=&quot;dare iussus&quot;>
               &lt;air:CountryCode>au&lt;/air:CountryCode>
               &lt;air:TaxCategory>iunonis adorat praeterea&lt;/air:TaxCategory>
            &lt;/air:ExemptTaxes>
            &lt;air:PenaltyFareInformation ProhibitPenaltyFares=&quot;false&quot;>
               &lt;air:PenaltyInfo PenaltyApplies=&quot;Before Departure&quot; NoShow=&quot;true&quot;>
                  &lt;air:Amount>foedere certo&lt;/air:Amount>
                  &lt;air:Percentage>verrantque per auras&lt;/air:Percentage>
               &lt;/air:PenaltyInfo>
            &lt;/air:PenaltyFareInformation>
            &lt;com:DiscountCard Key=&quot;hic vasto rex&quot; Code=&quot;per aura&quot; Description=&quot;vinclis et&quot; Number=&quot;montis insuper altos&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
            &lt;air:PromoCodes>
               &lt;air:PromoCode Code=&quot;secum dea&quot; ProviderCode=&quot;iras &quot; SupplierCode=&quot;et qu&quot;/>
            &lt;/air:PromoCodes>
            &lt;air:ManualFareAdjustment AppliedOn=&quot;Other&quot; AdjustmentType=&quot;Amount&quot; Value=&quot;1000.00&quot; PassengerRef=&quot;iras ni&quot; TicketDesignator=&quot;iras ni&quot; FareType=&quot;tempe&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;austr&quot; PseudoCityCode=&quot;divum ince&quot; Key=&quot;et quisquam&quot; IATA=&quot;et coniu&quot;/>
            &lt;air:BrandModifiers>
               &lt;air:FareFamilyDisplay ModifierType=&quot;tempestatesque sonoras&quot;/>
               &lt;air:BasicDetailsOnly ReturnBasicDetails=&quot;false&quot;/>
            &lt;/air:BrandModifiers>
            &lt;air:MultiGDSSearchIndicator Type=&quot;caelumque profundum&quot; ProviderCode=&quot;aeoli&quot; DefaultProvider=&quot;true&quot; PrivateFareCode=&quot;corripuit scopuloque&quot; PrivateFareCodeOnly=&quot;false&quot;/>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;et carcere frenat&quot;/>
            &lt;/air:PreferredCabins>
         &lt;/air:AirPricingModifiers>
         &lt;air:Enumeration>
            &lt;air:SolutionGroup Count=&quot;100&quot; TripType=&quot;Business&quot; Diversification=&quot;DateCombination&quot; Tag=&quot;claustra fremunt&quot; Primary=&quot;false&quot;>
               &lt;air:PermittedAccountCodes>
                  &lt;com:AccountCode Code=&quot;circum claustra&quot; ProviderCode=&quot;magno&quot; SupplierCode=&quot;gero &quot; Type=&quot;rapidum iaculata e&quot;/>
               &lt;/air:PermittedAccountCodes>
               &lt;air:PreferredAccountCodes>
                  &lt;com:AccountCode Code=&quot;et montis&quot; ProviderCode=&quot;laxas&quot; SupplierCode=&quot;auras&quot; Type=&quot;regina iovisque et&quot;/>
               &lt;/air:PreferredAccountCodes>
               &lt;air:ProhibitedAccountCodes>
                  &lt;com:AccountCode Code=&quot;pater omnipotens speluncis&quot; ProviderCode=&quot;corri&quot; SupplierCode=&quot;patri&quot; Type=&quot;infixit acuto&quot;/>
               &lt;/air:ProhibitedAccountCodes>
               &lt;air:PermittedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;frena&quot; PseudoCityCode=&quot;montis cir&quot; Key=&quot;iaculata e&quot; IATA=&quot;metuens &quot;/>
               &lt;/air:PermittedPointOfSales>
               &lt;air:ProhibitedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;animo&quot; PseudoCityCode=&quot;dea corde&quot; Key=&quot;tempestatesque sonoras&quot; IATA=&quot;evertitq&quot;/>
               &lt;/air:ProhibitedPointOfSales>
            &lt;/air:SolutionGroup>
         &lt;/air:Enumeration>
         &lt;air:AirExchangeModifiers BookingDate=&quot;et quisquam&quot; TicketingDate=&quot;flammas turbine&quot; AccountCode=&quot;transfixo pectore flammas&quot; TicketDesignator=&quot;cum gente&quot; AllowPenaltyFares=&quot;true&quot; PrivateFaresOnly=&quot;false&quot; UniversalRecordLocatorCode=&quot;frenat i&quot; ProviderLocatorCode=&quot;premere &quot; ProviderCode=&quot;iras ni&quot;>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;disiecitque rates&quot; CompanyName=&quot;cum murmure&quot; ProviderCode=&quot;vincl&quot; SupplierCode=&quot;dea c&quot;/>
            &lt;/air:ContractCodes>
         &lt;/air:AirExchangeModifiers>
         &lt;air:FlexExploreModifiers Type=&quot;Area&quot; Radius=&quot;100&quot; GroupName=&quot;ni faciat&quot;>
            &lt;air:Destination>fla&lt;/air:Destination>
         &lt;/air:FlexExploreModifiers>
         &lt;air:PCC>
            &lt;com:OverridePCC ProviderCode=&quot;facia&quot; PseudoCityCode=&quot;ignem disi&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;illum&quot; PseudoCityCode=&quot;et premere&quot; Key=&quot;volutans nimborum&quot; IATA=&quot;feta fur&quot;/>
            &lt;air:TicketAgency ProviderCode=&quot;anySimpleType&quot; PseudoCityCode=&quot;anySimpleType&quot;/>
         &lt;/air:PCC>
         &lt;air:FareRulesFilterCategory FareInfoRef=&quot;et temperat&quot;>
            &lt;air:CategoryCode>habenas ipsa&lt;/air:CategoryCode>
         &lt;/air:FareRulesFilterCategory>
         &lt;com:FormOfPayment Key=&quot;iras ni&quot; Type=&quot;fremunt celsa&quot; FulfillmentType=&quot;dea corde&quot; FulfillmentLocation=&quot;aris imponet honorem&quot; FulfillmentIDType=&quot;Collection Reference&quot; FulfillmentIDNumber=&quot;disiecitque rates evertitque&quot; IsAgentType=&quot;false&quot; AgentText=&quot;molemque et&quot; ReuseFOP=&quot;aut supplex&quot; ExternalReference=&quot;atris hoc&quot; Reusable=&quot;false&quot; ProfileID=&quot;flammato secum&quot; ProfileKey=&quot;sceptra tenens mollitque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:CreditCard Type=&quot;au&quot; Number=&quot;et montis insuper&quot; ExpDate=&quot;2001-09&quot; Name=&quot;vasto rex&quot; CVV=&quot;iuss&quot; ApprovalCode=&quot;supplex aris&quot; ExtendedPayment=&quot;adorat praeterea&quot; CustomerReference=&quot;et temperat iras&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;rapidi secum&quot; BankCountryCode=&quot;di&quot; BankStateCode=&quot;dea co&quot; Enett=&quot;false&quot; ProfileID=&quot;ferant rapidi&quot; Key=&quot;abdidit atris hoc&quot;>
               &lt;com:PhoneNumber Key=&quot;incedo regina&quot; Type=&quot;Other&quot; Location=&quot;coniunx un&quot; CountryCode=&quot;aequo&quot; AreaCode=&quot;disiecitqu&quot; Number=&quot;supplex aris&quot; Extension=&quot;atris hoc&quot; Text=&quot;abdidit atris&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;et temperat iras&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;ignem disiecitque rates&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>et montis&lt;/com:AddressName>
                  &lt;com:Street>imperio premit ac&lt;/com:Street>
                  &lt;com:City>secum dea&lt;/com:City>
                  &lt;com:State>regemque dedit&lt;/com:State>
                  &lt;com:PostalCode>corde volutans&lt;/com:PostalCode>
                  &lt;com:Country>vi&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;circum claustra fremunt&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;ci&quot; Number=&quot;pectore flammas&quot; ExpDate=&quot;2009-07&quot; Name=&quot;ni faciat&quot; CVV=&quot;tali&quot; ApprovalCode=&quot;nimborum in&quot; IssueNumber=&quot;imperio &quot; ProfileID=&quot;rapidi secum&quot; Key=&quot;illi indignantes magno&quot;>
               &lt;com:PhoneNumber Key=&quot;claustra fremunt&quot; Type=&quot;Email&quot; Location=&quot;ac vinclis&quot; CountryCode=&quot;haben&quot; AreaCode=&quot;sed pater &quot; Number=&quot;iovis rapidum iaculata&quot; Extension=&quot;ni faciat&quot; Text=&quot;iussus habenas&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;dedit qui foedere&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;ipsa iovis rapidum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>auras sed&lt;/com:AddressName>
                  &lt;com:Street>tenens mollitque&lt;/com:Street>
                  &lt;com:City>exspirantem transfixo&lt;/com:City>
                  &lt;com:State>exspirantem transfixo&lt;/com:State>
                  &lt;com:PostalCode>vinclis et&lt;/com:PostalCode>
                  &lt;com:Country>di&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;furentibus austris&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;claustra fremunt celsa&quot; Amount=&quot;et laxas sciret&quot; DiscountAmount=&quot;habenas ipsa&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2005-10-19+05:30&quot; NotValidAfter=&quot;2000-08-11&quot;/>
            &lt;com:TicketNumber>ego quae&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;dea corde volutans&quot; RoutingNumber=&quot;celsa sedet&quot; AccountNumber=&quot;flammato secum&quot; CheckNumber=&quot;et laxas&quot;/>
            &lt;com:Requisition Number=&quot;pater omnipotens&quot; Category=&quot;Other&quot; Type=&quot;Credit&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;ra&quot; CreditCardNumber=&quot;e nubibuse nubibus&quot; ExpDate=&quot;2015-02&quot; Text=&quot;corde volutans&quot; Category=&quot;iaculata e&quot; AcceptanceOverride=&quot;false&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;et coniunx una&quot; AgencyBillingNumber=&quot;in patriam loca&quot; AgencyBillingPassword=&quot;quippe ferant&quot;/>
            &lt;com:UnitedNations Number=&quot;ni faciat&quot;/>
            &lt;com:DirectPayment Text=&quot;ventis illum&quot;/>
            &lt;com:AgentVoucher Number=&quot;foedere certo&quot;/>
            &lt;com:PaymentAdvice Type=&quot;rap&quot; DocumentNumber=&quot;quippe ferant&quot; IssueDate=&quot;2006-04-03+05:30&quot; IssueCity=&quot;et &quot; OriginalFOP=&quot;illi indignantes ma&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;et premere&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;claustra fremunt&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;premit ac&quot; BSPPassword=&quot;talia flammato secum&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;omnipotens speluncis&quot; ARCPassword=&quot;verrantque per&quot;/>
         &lt;/com:FormOfPayment>
      &lt;/air:LowFareSearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/air_v51_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
