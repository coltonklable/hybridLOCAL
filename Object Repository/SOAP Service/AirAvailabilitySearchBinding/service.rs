<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>b8445083-dc95-47c3-9c4e-da246045d1b2</elementGuidId>
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
         &lt;ses:SessTok id=&quot;in patriam loca&quot;/>
         &lt;ses:SessProp nm=&quot;talia flammato&quot; val=&quot;auras sed&quot;/>
      &lt;/ses:SessionContext>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;air:AvailabilitySearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; ReturnBrandIndicator=&quot;false&quot; ChannelId=&quot;rege&quot; NSCC=&quot;fre&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;ferant rapidi&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;scept&quot; ProviderCode=&quot;turbi&quot; AgentID=&quot;flammato secum dea&quot;/>
         &lt;com:TerminalSessionInfo>profundum quippe ferant&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;nubib&quot; PseudoCityCode=&quot;ac vinclis&quot;/>
         &lt;com:NextResultReference ProviderCode=&quot;rapid&quot;>caelumque profundum&lt;/com:NextResultReference>
         &lt;com:PointOfCommencement CityOrAirportCode=&quot;cir&quot; Time=&quot;coniunx una&quot;/>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:Airport Code=&quot;arc&quot;/>
               &lt;com:City Code=&quot;cir&quot;/>
               &lt;com:CityOrAirport Code=&quot;cer&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;et monti&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;fe&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:Airport Code=&quot;sed&quot;/>
               &lt;com:City Code=&quot;tal&quot;/>
               &lt;com:CityOrAirport Code=&quot;aus&quot; PreferCity=&quot;false&quot;/>
               &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
               &lt;com:RailLocation Code=&quot;sciret d&quot;/>
               &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;da&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime PreferredTime=&quot;bella gero&quot;>
               &lt;com:TimeRange EarliestTime=&quot;dedit qui foedere&quot; LatestTime=&quot;magno cum&quot;/>
               &lt;com:SpecificTime Time=&quot;ac terras&quot;/>
               &lt;com:SearchExtraDays DaysBefore=&quot;3&quot; DaysAfter=&quot;3&quot;/>
            &lt;/air:SearchDepTime>
            &lt;air:SearchArvTime PreferredTime=&quot;regemque dedit&quot;>
               &lt;com:TimeRange EarliestTime=&quot;infixit acuto&quot; LatestTime=&quot;tenens mollitque animos&quot;/>
               &lt;com:SpecificTime Time=&quot;montis insuper&quot;/>
            &lt;/air:SearchArvTime>
            &lt;air:AirLegModifiers ProhibitOvernightLayovers=&quot;false&quot; MaxConnectionTime=&quot;100&quot; ReturnFirstAvailableOnly=&quot;true&quot; AllowDirectAccess=&quot;false&quot; ProhibitMultiAirportConnection=&quot;true&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;JourneyTime&quot; MaxJourneyTime=&quot;99&quot;>
               &lt;air:PermittedCabins>
                  &lt;com:CabinClass Type=&quot;cum murmure&quot;/>
               &lt;/air:PermittedCabins>
               &lt;air:PreferredCabins>
                  &lt;com:CabinClass Type=&quot;in patriam&quot;/>
               &lt;/air:PreferredCabins>
               &lt;air:PermittedCarriers>
                  &lt;com:Carrier Code=&quot;ab&quot;/>
               &lt;/air:PermittedCarriers>
               &lt;air:ProhibitedCarriers>
                  &lt;com:Carrier Code=&quot;co&quot;/>
               &lt;/air:ProhibitedCarriers>
               &lt;air:PreferredCarriers>
                  &lt;com:Carrier Code=&quot;ip&quot;/>
               &lt;/air:PreferredCarriers>
               &lt;air:PermittedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;fre&quot;/>
                     &lt;com:City Code=&quot;nim&quot;/>
                     &lt;com:CityOrAirport Code=&quot;mol&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PermittedConnectionPoints>
               &lt;air:ProhibitedConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;mon&quot;/>
                     &lt;com:City Code=&quot;mol&quot;/>
                     &lt;com:CityOrAirport Code=&quot;ger&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:ProhibitedConnectionPoints>
               &lt;air:PreferredConnectionPoints>
                  &lt;com:ConnectionPoint>
                     &lt;com:Airport Code=&quot;cir&quot;/>
                     &lt;com:City Code=&quot;mon&quot;/>
                     &lt;com:CityOrAirport Code=&quot;ari&quot; PreferCity=&quot;false&quot;/>
                  &lt;/com:ConnectionPoint>
               &lt;/air:PreferredConnectionPoints>
               &lt;air:PermittedBookingCodes>
                  &lt;air:BookingCode Code=&quot;ar&quot;/>
               &lt;/air:PermittedBookingCodes>
               &lt;air:PreferredBookingCodes>
                  &lt;air:BookingCode Code=&quot;in&quot;/>
               &lt;/air:PreferredBookingCodes>
               &lt;air:PreferredAlliances>
                  &lt;air:Alliance Code=&quot;ac vinclis&quot;/>
               &lt;/air:PreferredAlliances>
               &lt;air:ProhibitedBookingCodes>
                  &lt;air:BookingCode Code=&quot;ac&quot;/>
               &lt;/air:ProhibitedBookingCodes>
               &lt;air:DisfavoredAlliances>
                  &lt;air:Alliance Code=&quot;et premere et&quot;/>
               &lt;/air:DisfavoredAlliances>
               &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;false&quot; StopDirects=&quot;true&quot; SingleOnlineCon=&quot;false&quot; DoubleOnlineCon=&quot;true&quot; TripleOnlineCon=&quot;true&quot; SingleInterlineCon=&quot;true&quot; DoubleInterlineCon=&quot;true&quot; TripleInterlineCon=&quot;true&quot;/>
               &lt;air:AnchorFlightData AirlineCode=&quot;ir&quot; FlightNumber=&quot;verra&quot; ConnectionIndicator=&quot;false&quot;/>
            &lt;/air:AirLegModifiers>
         &lt;/air:SearchAirLeg>
         &lt;air:SearchSpecificAirSegment DepartureTime=&quot;montis circum&quot; Carrier=&quot;la&quot; FlightNumber=&quot;e nub&quot; Origin=&quot;vol&quot; Destination=&quot;mur&quot; SegmentIndex=&quot;3&quot;/>
         &lt;air:AirSearchModifiers DistanceType=&quot;MI&quot; IncludeFlightDetails=&quot;true&quot; AllowChangeOfAirport=&quot;true&quot; ProhibitOvernightLayovers=&quot;false&quot; MaxSolutions=&quot;100&quot; MaxConnectionTime=&quot;100&quot; SearchWeekends=&quot;false&quot; IncludeExtraSolutions=&quot;false&quot; ProhibitMultiAirportConnection=&quot;false&quot; PreferNonStop=&quot;false&quot; OrderBy=&quot;DepartureTime&quot; ExcludeOpenJawAirport=&quot;false&quot; ExcludeGroundTransportation=&quot;false&quot; MaxJourneyTime=&quot;99&quot; JetServiceOnly=&quot;true&quot;>
            &lt;air:DisfavoredProviders>
               &lt;com:Provider Code=&quot;auras&quot;/>
            &lt;/air:DisfavoredProviders>
            &lt;air:PreferredProviders>
               &lt;com:Provider Code=&quot;aequo&quot;/>
            &lt;/air:PreferredProviders>
            &lt;air:DisfavoredCarriers>
               &lt;com:Carrier Code=&quot;ae&quot;/>
            &lt;/air:DisfavoredCarriers>
            &lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;et&quot;/>
            &lt;/air:PermittedCarriers>
            &lt;air:ProhibitedCarriers>
               &lt;com:Carrier Code=&quot;in&quot;/>
            &lt;/air:ProhibitedCarriers>
            &lt;air:PreferredCarriers>
               &lt;com:Carrier Code=&quot;hi&quot;/>
            &lt;/air:PreferredCarriers>
            &lt;air:PermittedCabins>
               &lt;com:CabinClass Type=&quot;ego quae&quot;/>
            &lt;/air:PermittedCabins>
            &lt;air:PreferredCabins>
               &lt;com:CabinClass Type=&quot;tempestatesque sonoras&quot;/>
            &lt;/air:PreferredCabins>
            &lt;air:PreferredAlliances>
               &lt;air:Alliance Code=&quot;ego quae divum&quot;/>
            &lt;/air:PreferredAlliances>
            &lt;air:DisfavoredAlliances>
               &lt;air:Alliance Code=&quot;aris imponet honorem&quot;/>
            &lt;/air:DisfavoredAlliances>
            &lt;air:PermittedBookingCodes>
               &lt;air:BookingCode Code=&quot;mo&quot;/>
            &lt;/air:PermittedBookingCodes>
            &lt;air:PreferredBookingCodes>
               &lt;air:BookingCode Code=&quot;ra&quot;/>
            &lt;/air:PreferredBookingCodes>
            &lt;air:ProhibitedBookingCodes>
               &lt;air:BookingCode Code=&quot;ni&quot;/>
            &lt;/air:ProhibitedBookingCodes>
            &lt;air:FlightType RequireSingleCarrier=&quot;false&quot; MaxConnections=&quot;-1&quot; MaxStops=&quot;-1&quot; NonStopDirects=&quot;false&quot; StopDirects=&quot;false&quot; SingleOnlineCon=&quot;false&quot; DoubleOnlineCon=&quot;true&quot; TripleOnlineCon=&quot;false&quot; SingleInterlineCon=&quot;true&quot; DoubleInterlineCon=&quot;false&quot; TripleInterlineCon=&quot;true&quot;/>
            &lt;air:MaxLayoverDuration Domestic=&quot;100&quot; Gateway=&quot;100&quot; International=&quot;100&quot;/>
            &lt;air:NativeSearchModifier ProviderCode=&quot;carce&quot;>murmure montis circum&lt;/air:NativeSearchModifier>
         &lt;/air:AirSearchModifiers>
         &lt;air:JourneyData>
            &lt;air:AirSegment Key=&quot;maria ac&quot; Status=&quot;imponet honorem talia&quot; Passive=&quot;false&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; OpenSegment=&quot;true&quot; Group=&quot;3&quot; Carrier=&quot;et&quot; CabinClass=&quot;tempestatesque sonoras&quot; FlightNumber=&quot;ego q&quot; Origin=&quot;iac&quot; Destination=&quot;omn&quot; DepartureTime=&quot;premere et laxas&quot; ArrivalTime=&quot;talia flammato&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; ProviderCode=&quot;terra&quot; SupplierCode=&quot;et ca&quot; ParticipantLevel=&quot;dare iussus&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;illi indignantes magno&quot; AvailabilityDisplayType=&quot;dea corde&quot; ClassOfService=&quot;ra&quot; ETicketability=&quot;Ticketless&quot; Equipment=&quot;ac &quot; MarriageGroup=&quot;3&quot; NumberOfStops=&quot;3&quot; Seamless=&quot;true&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;e nubibus&quot; HostTokenRef=&quot;tenens mollitque&quot; ProviderReservationInfoRef=&quot;vinclis et carcere&quot; PassiveProviderReservationInfoRef=&quot;aeolus antro&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;r&quot; APISRequirementsRef=&quot;rapidum iaculata&quot; BlackListed=&quot;false&quot; OperationalStatus=&quot;transfixo pectore flammas&quot; NumberInParty=&quot;99&quot; RailCoachNumber=&quot;ego &quot; BookingDate=&quot;2004-12-18&quot; FlownSegment=&quot;false&quot; ScheduleChange=&quot;false&quot; BrandIndicator=&quot;iovis rapidum&quot;>
               &lt;com:SegmentRemark Key=&quot;indignantes magno&quot;>sonoras imperio&lt;/com:SegmentRemark>
               &lt;air:SponsoredFltInfo SponsoredLNB=&quot;200&quot; NeutralLNB=&quot;200&quot; FltKey=&quot;rapid&quot;/>
               &lt;air:CodeshareInfo OperatingCarrier=&quot;au&quot; OperatingFlightNumber=&quot;iovis&quot;>flammato secum dea&lt;/air:CodeshareInfo>
               &lt;air:AirAvailInfo ProviderCode=&quot;hic v&quot; HostTokenRef=&quot;aeoliam venit&quot;>
                  &lt;air:BookingCodeInfo CabinClass=&quot;altos imposuit regemque&quot; BookingCounts=&quot;profundum quippe&quot;/>
                  &lt;air:FareTokenInfo FareInfoRef=&quot;premit ac vinclis&quot; HostTokenRef=&quot;luctantis ventos&quot;/>
               &lt;/air:AirAvailInfo>
               &lt;air:FlightDetails Key=&quot;luctantis ventos tempestatesque&quot; Origin=&quot;nub&quot; Destination=&quot;et &quot; DepartureTime=&quot;annos bella gero&quot; ArrivalTime=&quot;ignem disiecitque&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; Equipment=&quot;iov&quot; OnTimePerformance=&quot;100&quot; OriginTerminal=&quot;molemque et montis&quot; DestinationTerminal=&quot;montis insuper&quot; GroundTime=&quot;100&quot; AutomatedCheckin=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;NoStopOver&quot;>
                     &lt;air:FareNote Key=&quot;omnipotens speluncis abdidit&quot; Precedence=&quot;100&quot; NoteName=&quot;animos et temperat&quot; FareInfoMessageRef=&quot;sciret dare&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>dea corde&lt;/air:FareNote>
                  &lt;/air:Connection>
                  &lt;air:Meals>Dinner&lt;/air:Meals>
                  &lt;air:InFlightServices>claustra fremunt&lt;/air:InFlightServices>
               &lt;/air:FlightDetails>
               &lt;air:FlightDetailsRef Key=&quot;una cum&quot;/>
               &lt;air:AlternateLocationDistanceRef Key=&quot;corde volutans nimborum&quot;/>
               &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;IgnoreSegment&quot;>
                  &lt;air:FareNote Key=&quot;sciret dare iussus&quot; Precedence=&quot;100&quot; NoteName=&quot;ac terras caelumque&quot; FareInfoMessageRef=&quot;regina iovisque et&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>terras caelumque&lt;/air:FareNote>
               &lt;/air:Connection>
               &lt;com:SellMessage>aut supplex aris&lt;/com:SellMessage>
               &lt;air:RailCoachDetails RailCoachNumber=&quot;regemque dedit&quot; AvailableRailSeats=&quot;iras ni&quot; RailSeatMapAvailability=&quot;true&quot;/>
            &lt;/air:AirSegment>
         &lt;/air:JourneyData>
         &lt;com:SearchPassenger Code=&quot;ignem&quot; Age=&quot;100&quot; DOB=&quot;2011-05-25&quot; Gender=&quot;va&quot; PricePTCOnly=&quot;true&quot; BookingTravelerRef=&quot;mollitque animos&quot; AccompaniedPassenger=&quot;false&quot; ResidencyType=&quot;Employee&quot; Key=&quot;et carcere&quot;>
            &lt;com:Name Prefix=&quot;iaculata e&quot; First=&quot;divum incedo regina&quot; Middle=&quot;insuper altos&quot; Last=&quot;imperio premit&quot; Suffix=&quot;hic vasto rex&quot; TravelerProfileId=&quot;100&quot;/>
            &lt;com:LoyaltyCard Key=&quot;murmure montis&quot; SupplierCode=&quot;il&quot; AllianceLevel=&quot;tot annos bella&quot; MembershipProgram=&quot;profundum quippe&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; CardNumber=&quot;murmure montis&quot; Status=&quot;numen iunonis adorat&quot; MembershipStatus=&quot;ventos tempestatesque sonoras&quot; FreeText=&quot;in patriam loca&quot; SupplierType=&quot;Other&quot; Level=&quot;quippe ferant&quot; PriorityCode=&quot;sonoras imperio&quot; VendorLocationRef=&quot;illi indignantes&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;false&quot; ReservationLevel=&quot;true&quot;>
                  &lt;com:OperatedBy>molemque et montis&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;mollitque animos et&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;tenens mollitque&quot; Code=&quot;circum c&quot; Description=&quot;circum claustra fremunt&quot; Number=&quot;acuto ast&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
            &lt;com:PersonalGeography>
               &lt;com:CountryCode>ac&lt;/com:CountryCode>
               &lt;com:StateProvinceCode>premer&lt;/com:StateProvinceCode>
               &lt;com:CityCode>fla&lt;/com:CityCode>
            &lt;/com:PersonalGeography>
         &lt;/com:SearchPassenger>
         &lt;com:PointOfSale ProviderCode=&quot;ipsa &quot; PseudoCityCode=&quot;sedet aeol&quot; Key=&quot;frenat illi&quot; IATA=&quot;vinclis &quot;/>
      &lt;/air:AvailabilitySearchReq>
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
