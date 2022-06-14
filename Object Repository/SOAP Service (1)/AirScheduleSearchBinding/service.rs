<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>d62a8a33-e4a3-4211-b201-bdf33640bc70</elementGuidId>
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
      &lt;air:ScheduleSearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;com:NextResultReference ProviderCode=&quot;nubib&quot;>ac vinclis et&lt;/com:NextResultReference>
         &lt;com:PointOfCommencement CityOrAirportCode=&quot;rap&quot; Time=&quot;caelumque profundum&quot;/>
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
      &lt;/air:ScheduleSearchReq>
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
