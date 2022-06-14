<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>7b32ec6e-3bf5-4618-b07d-9c868ec8a249</elementGuidId>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ses=&quot;http://www.travelport.com/soa/common/security/SessionContext_v1&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v49_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v49_0&quot;>
   &lt;soapenv:Header>
      &lt;ses:SessionContext>
         &lt;ses:SessTok id=&quot;omnipotens speluncis&quot;/>
         &lt;ses:SessProp nm=&quot;verrantque per&quot; val=&quot;una cum gente&quot;/>
      &lt;/ses:SessionContext>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;air:LowFareSearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; EnablePointToPointSearch=&quot;false&quot; EnablePointToPointAlternates=&quot;false&quot; MaxNumberOfExpertSolutions=&quot;0&quot; SolutionResult=&quot;false&quot; PreferCompleteItinerary=&quot;true&quot; MetaOptionIdentifier=&quot;re&quot; ReturnUpsellFare=&quot;false&quot; IncludeFareInfoMessages=&quot;false&quot; ReturnBrandedFares=&quot;true&quot; MultiGDSSearch=&quot;false&quot; ReturnMM=&quot;false&quot; CheckOBFees=&quot;fremunt celsa&quot; NSCC=&quot;fer&quot; FareInfoRules=&quot;false&quot; PolicyReference=&quot;sceptra tenens&quot;>
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
         &lt;air:AirPricingModifiers ProhibitMinStayFares=&quot;false&quot; ProhibitMaxStayFares=&quot;false&quot; CurrencyType=&quot;pre&quot; ProhibitAdvancePurchaseFares=&quot;false&quot; ProhibitNonRefundableFares=&quot;false&quot; ProhibitRestrictedFares=&quot;false&quot; FaresIndicator=&quot;PrivateFaresOnly&quot; FiledCurrency=&quot;pro&quot; PlatingCarrier=&quot;ve&quot; OverrideCarrier=&quot;ig&quot; ETicketability=&quot;No&quot; AccountCodeFaresOnly=&quot;true&quot; Key=&quot;supplex aris&quot; ProhibitNonExchangeableFares=&quot;false&quot; ForceSegmentSelect=&quot;false&quot; InventoryRequestType=&quot;DirectAccess&quot; OneWayShop=&quot;false&quot; ProhibitUnbundledFareTypes=&quot;true&quot; ReturnServices=&quot;true&quot; ChannelId=&quot;indi&quot; ReturnFareAttributes=&quot;false&quot; SellCheck=&quot;false&quot; ReturnFailedSegments=&quot;false&quot;>
            &lt;air:ProhibitedRuleCategories>
               &lt;air:FareRuleCategory Category=&quot;50&quot;/>
            &lt;/air:ProhibitedRuleCategories>
            &lt;air:AccountCodes>
               &lt;com:AccountCode Code=&quot;ventis illum&quot; ProviderCode=&quot;carce&quot; SupplierCode=&quot;flamm&quot; Type=&quot;habenas ipsa&quot;/>
            &lt;/air:AccountCodes>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;caelumque profundum quippe&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;tenens mollitque&quot; CompanyName=&quot;rates evertitque aequora&quot; ProviderCode=&quot;laxas&quot; SupplierCode=&quot;gente&quot;/>
            &lt;/air:ContractCodes>
            &lt;air:ExemptTaxes AllTaxes=&quot;true&quot; TaxTerritory=&quot;ab&quot; CompanyName=&quot;aequora ventis illum&quot;>
               &lt;air:CountryCode>ex&lt;/air:CountryCode>
               &lt;air:TaxCategory>dare iussus&lt;/air:TaxCategory>
            &lt;/air:ExemptTaxes>
            &lt;air:PenaltyFareInformation ProhibitPenaltyFares=&quot;true&quot;>
               &lt;air:PenaltyInfo PenaltyApplies=&quot;Before Departure&quot; NoShow=&quot;true&quot;>
                  &lt;air:Amount>ventos tempestatesque&lt;/air:Amount>
                  &lt;air:Percentage>et temperat&lt;/air:Percentage>
               &lt;/air:PenaltyInfo>
            &lt;/air:PenaltyFareInformation>
            &lt;com:DiscountCard Key=&quot;foedere certo&quot; Code=&quot;verrantq&quot; Description=&quot;hic vasto rex&quot; Number=&quot;per auras sed&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;/>
            &lt;air:PromoCodes>
               &lt;air:PromoCode Code=&quot;montis insuper altos&quot; ProviderCode=&quot;acuto&quot; SupplierCode=&quot;secum&quot;/>
            &lt;/air:PromoCodes>
            &lt;air:ManualFareAdjustment AppliedOn=&quot;Base&quot; AdjustmentType=&quot;Percentage&quot; Value=&quot;1000.00&quot; PassengerRef=&quot;et quisquam&quot; TicketDesignator=&quot;carcere frenat illi&quot; FareType=&quot;iras &quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;iras &quot; PseudoCityCode=&quot;tempestate&quot; Key=&quot;austris aeoliam&quot; IATA=&quot;divum in&quot;/>
            &lt;air:BrandModifiers>
               &lt;air:FareFamilyDisplay ModifierType=&quot;et quisquam&quot;/>
               &lt;air:BasicDetailsOnly ReturnBasicDetails=&quot;false&quot;/>
            &lt;/air:BrandModifiers>
            &lt;air:MultiGDSSearchIndicator Type=&quot;aeoliam venit hic&quot; ProviderCode=&quot;talia&quot; DefaultProvider=&quot;false&quot; PrivateFareCode=&quot;murmure montis&quot; PrivateFareCodeOnly=&quot;true&quot;/>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;nubibus ignem&quot;/>
            &lt;/air:PreferredCabins>
         &lt;/air:AirPricingModifiers>
         &lt;air:Enumeration>
            &lt;air:SolutionGroup Count=&quot;100&quot; TripType=&quot;Quickest&quot; Diversification=&quot;Airports&quot; Tag=&quot;et carcere frenat&quot; Primary=&quot;false&quot;>
               &lt;air:PermittedAccountCodes>
                  &lt;com:AccountCode Code=&quot;austris aeoliam&quot; ProviderCode=&quot;claus&quot; SupplierCode=&quot;circu&quot; Type=&quot;magno cum&quot;/>
               &lt;/air:PermittedAccountCodes>
               &lt;air:PreferredAccountCodes>
                  &lt;com:AccountCode Code=&quot;gero et&quot; ProviderCode=&quot;rapid&quot; SupplierCode=&quot;et mo&quot; Type=&quot;laxas sciret dare&quot;/>
               &lt;/air:PreferredAccountCodes>
               &lt;air:ProhibitedAccountCodes>
                  &lt;com:AccountCode Code=&quot;auras sed&quot; ProviderCode=&quot;regin&quot; SupplierCode=&quot;pater&quot; Type=&quot;corripuit scopuloque infixit&quot;/>
               &lt;/air:ProhibitedAccountCodes>
               &lt;air:PermittedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;patri&quot; PseudoCityCode=&quot;infixit ac&quot; Key=&quot;frenat illi&quot; IATA=&quot;montis c&quot;/>
               &lt;/air:PermittedPointOfSales>
               &lt;air:ProhibitedPointOfSales>
                  &lt;com:PointOfSale ProviderCode=&quot;iacul&quot; PseudoCityCode=&quot;metuens mo&quot; Key=&quot;animos et&quot; IATA=&quot;dea cord&quot;/>
               &lt;/air:ProhibitedPointOfSales>
            &lt;/air:SolutionGroup>
         &lt;/air:Enumeration>
         &lt;air:AirExchangeModifiers BookingDate=&quot;tempestatesque sonoras&quot; TicketingDate=&quot;evertitque aequora ventis&quot; AccountCode=&quot;et quisquam&quot; TicketDesignator=&quot;flammas turbine&quot; AllowPenaltyFares=&quot;true&quot; PrivateFaresOnly=&quot;false&quot; UniversalRecordLocatorCode=&quot;transfix&quot; ProviderLocatorCode=&quot;cum gent&quot; ProviderCode=&quot;frenat illi&quot;>
            &lt;air:ContractCodes>
               &lt;air:ContractCode Code=&quot;premere et&quot; CompanyName=&quot;iras ni&quot; ProviderCode=&quot;disie&quot; SupplierCode=&quot;cum m&quot;/>
            &lt;/air:ContractCodes>
         &lt;/air:AirExchangeModifiers>
         &lt;air:FlexExploreModifiers Type=&quot;Zone&quot; Radius=&quot;100&quot; GroupName=&quot;aeolus arce sce&quot;>
            &lt;air:Destination>imp&lt;/air:Destination>
         &lt;/air:FlexExploreModifiers>
         &lt;air:PCC>
            &lt;com:OverridePCC ProviderCode=&quot;ni fa&quot; PseudoCityCode=&quot;flammato s&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;facia&quot; PseudoCityCode=&quot;ignem disi&quot; Key=&quot;illum exspirantem&quot; IATA=&quot;et preme&quot;/>
            &lt;air:TicketAgency ProviderCode=&quot;anySimpleType&quot; PseudoCityCode=&quot;anySimpleType&quot;/>
         &lt;/air:PCC>
         &lt;air:FareRulesFilterCategory FareInfoRef=&quot;volutans nimborum&quot;>
            &lt;air:CategoryCode>feta furentibus&lt;/air:CategoryCode>
         &lt;/air:FareRulesFilterCategory>
         &lt;com:FormOfPayment Key=&quot;et temperat&quot; Type=&quot;habenas ipsa&quot; FulfillmentType=&quot;iras ni&quot; FulfillmentLocation=&quot;fremunt celsa&quot; FulfillmentIDType=&quot;Euro Cheque Card&quot; FulfillmentIDNumber=&quot;furentibus austris aeoliam&quot; IsAgentType=&quot;false&quot; AgentText=&quot;corde volutans&quot; ReuseFOP=&quot;disiecitque rates evertitque&quot; ExternalReference=&quot;molemque et&quot; Reusable=&quot;false&quot; ProfileID=&quot;aut supplex&quot; ProfileKey=&quot;atris hoc&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:CreditCard Type=&quot;sc&quot; Number=&quot;fremunt celsa&quot; ExpDate=&quot;2003-06&quot; Name=&quot;foedere certo et&quot; CVV=&quot;flam&quot; ApprovalCode=&quot;vasto rex&quot; ExtendedPayment=&quot;iussus habenas&quot; CustomerReference=&quot;supplex aris&quot; AcceptanceOverride=&quot;false&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;et montis&quot; BankCountryCode=&quot;sp&quot; BankStateCode=&quot;rapidi&quot; Enett=&quot;false&quot; ProfileID=&quot;disiecitque rates&quot; Key=&quot;dea corde volutans&quot;>
               &lt;com:PhoneNumber Key=&quot;ferant rapidi&quot; Type=&quot;Hotel&quot; Location=&quot;celsa sede&quot; CountryCode=&quot;exspi&quot; AreaCode=&quot;coniunx un&quot; Number=&quot;aequora ventis illum&quot; Extension=&quot;disiecitqu&quot; Text=&quot;supplex aris&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;abdidit atris&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;dare iussus&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>ignem disiecitque rates&lt;/com:AddressName>
                  &lt;com:Street>rex aeolus antro&lt;/com:Street>
                  &lt;com:City>et montis&lt;/com:City>
                  &lt;com:State>imperio premit ac&lt;/com:State>
                  &lt;com:PostalCode>secum dea&lt;/com:PostalCode>
                  &lt;com:Country>re&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;corde volutans&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;vi&quot; Number=&quot;circum claustra fremunt&quot; ExpDate=&quot;2014-05+05:30&quot; Name=&quot;dare iussus habenas&quot; CVV=&quot;vent&quot; ApprovalCode=&quot;ni faciat&quot; IssueNumber=&quot;talia fl&quot; ProfileID=&quot;nimborum in&quot; Key=&quot;imperio premit&quot;>
               &lt;com:PhoneNumber Key=&quot;rapidi secum&quot; Type=&quot;Fax&quot; Location=&quot;aeolus arc&quot; CountryCode=&quot;illi &quot; AreaCode=&quot;ac vinclis&quot; Number=&quot;habenas ipsa&quot; Extension=&quot;sed pater &quot; Text=&quot;iovis rapidum iaculata&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;iussus habenas&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;ipsa iovis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>ipsa iovis rapidum&lt;/com:AddressName>
                  &lt;com:Street>patriam loca&lt;/com:Street>
                  &lt;com:City>auras sed&lt;/com:City>
                  &lt;com:State>tenens mollitque&lt;/com:State>
                  &lt;com:PostalCode>exspirantem tra&lt;/com:PostalCode>
                  &lt;com:Country>ex&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;vinclis et&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;furentibus austris&quot; Amount=&quot;volutans nimborum&quot; DiscountAmount=&quot;claustra fremunt celsa&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2016-01-15&quot; NotValidAfter=&quot;2015-02-15+05:30&quot;/>
            &lt;com:TicketNumber>premere et&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;et montis&quot; RoutingNumber=&quot;ego quae&quot; AccountNumber=&quot;dea corde volutans&quot; CheckNumber=&quot;celsa sedet&quot;/>
            &lt;com:Requisition Number=&quot;flammato secum&quot; Category=&quot;Other&quot; Type=&quot;Credit&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;pa&quot; CreditCardNumber=&quot;vasto rexvasto rex&quot; ExpDate=&quot;2016-09+05:30&quot; Text=&quot;infixit acuto ast&quot; Category=&quot;ipsa iovis&quot; AcceptanceOverride=&quot;false&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;molemque et&quot; AgencyBillingNumber=&quot;altos imposuit&quot; AgencyBillingPassword=&quot;et coniunx una&quot;/>
            &lt;com:UnitedNations Number=&quot;in patriam loca&quot;/>
            &lt;com:DirectPayment Text=&quot;quippe ferant&quot;/>
            &lt;com:AgentVoucher Number=&quot;ni faciat&quot;/>
            &lt;com:PaymentAdvice Type=&quot;ven&quot; DocumentNumber=&quot;foedere certo&quot; IssueDate=&quot;2009-06-21&quot; IssueCity=&quot;imp&quot; OriginalFOP=&quot;abdidit atris hoc&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;et laxas&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;illi indignantes magno&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;et premere&quot; BSPPassword=&quot;claustra fremunt&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;premit ac&quot; ARCPassword=&quot;talia flammato secum&quot;/>
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
   <wsdlAddress>file:/C:/Users/ramraja.sampathkumar/Desktop/SVT/POC_REST_SOAP/UAPI/wsdl/air_v49_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
