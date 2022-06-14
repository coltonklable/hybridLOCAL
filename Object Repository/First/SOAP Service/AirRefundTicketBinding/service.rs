<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>f3f8b29c-5994-4160-b1ae-2e1859653f75</elementGuidId>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;air:AirRefundReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;air:AirRefundBundle TicketNumber=&quot;nubibus ignem disiecitque&quot; PTC=&quot;anySimpleType&quot; RefundType=&quot;Manual&quot;>
            &lt;air:AirRefundInfo RefundAmount=&quot;hoc metuens&quot; RetainAmount=&quot;ac vinclis&quot; RefundFee=&quot;speluncis abdidit&quot; RefundableTaxes=&quot;aris imponet honorem&quot; FiledCurrency=&quot;pra&quot; ConversionRate=&quot;1000.00&quot; Taxes=&quot;claustra fremunt&quot; OriginalTicketTotal=&quot;imperio premit&quot; ForfeitAmount=&quot;quisquam numen&quot; Retain=&quot;false&quot; Refund=&quot;false&quot;>
               &lt;com:RefundRemark>
                  &lt;com:RemarkData>ac vinclis&lt;/com:RemarkData>
               &lt;/com:RefundRemark>
            &lt;/air:AirRefundInfo>
            &lt;com:Name Prefix=&quot;ac vinclis&quot; First=&quot;pectore flammas&quot; Middle=&quot;pectore flammas&quot; Last=&quot;annos bella gero&quot; Suffix=&quot;certo et&quot; TravelerProfileId=&quot;100&quot;/>
            &lt;air:TaxInfo Key=&quot;rates evertitque aequora&quot; Category=&quot;volutans nimborum in&quot; CarrierDefinedCategory=&quot;faciat maria&quot; SegmentRef=&quot;et quisquam&quot; FlightDetailsRef=&quot;et soror&quot; CouponRef=&quot;annos bella gero&quot; Amount=&quot;ventos tempestatesque sonoras&quot; OriginAirport=&quot;reg&quot; DestinationAirport=&quot;rap&quot; CountryCode=&quot;insuper altos&quot; FareInfoRef=&quot;cum murmure&quot; TaxExempted=&quot;true&quot; ProviderCode=&quot;soror&quot; SupplierCode=&quot;iunon&quot; Text=&quot;adorat praeterea&quot;>
               &lt;com:TaxDetail Amount=&quot;illum exspirantem transfixo&quot; OriginAirport=&quot;ari&quot; DestinationAirport=&quot;ari&quot; CountryCode=&quot;premere et&quot; FareInfoRef=&quot;insuper altos&quot;/>
               &lt;com:IncludedInBase Amount=&quot;ferant rapidi secum&quot;/>
            &lt;/air:TaxInfo>
            &lt;air:WaiverCode TourCode=&quot;nimborum in&quot; TicketDesignator=&quot;vasto rex&quot; Endorsement=&quot;flammato secum&quot;/>
         &lt;/air:AirRefundBundle>
         &lt;air:TCRRefundBundle TCRNumber=&quot;tempestatesque sonoras&quot; RefundType=&quot;Auto&quot; air:RefundAccessCode=&quot;incedo regina&quot;>
            &lt;air:AirRefundInfo RefundAmount=&quot;ac vinclis&quot; RetainAmount=&quot;acuto ast&quot; RefundFee=&quot;et premere et&quot; RefundableTaxes=&quot;dedit qui&quot; FiledCurrency=&quot;pro&quot; ConversionRate=&quot;1000.00&quot; Taxes=&quot;imposuit regemque&quot; OriginalTicketTotal=&quot;magno cum&quot; ForfeitAmount=&quot;iras ni faciat&quot; Retain=&quot;false&quot; Refund=&quot;false&quot;>
               &lt;com:RefundRemark>
                  &lt;com:RemarkData>verrantque per&lt;/com:RemarkData>
               &lt;/com:RefundRemark>
            &lt;/air:AirRefundInfo>
            &lt;air:WaiverCode TourCode=&quot;metuens molemqu&quot; TicketDesignator=&quot;dare iussus habenas&quot; Endorsement=&quot;tempestatesque sonoras imperio&quot;/>
            &lt;air:AirSegment Key=&quot;sceptra tenens mollitque&quot; Status=&quot;vasto rex&quot; Passive=&quot;false&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; OpenSegment=&quot;false&quot; Group=&quot;3&quot; Carrier=&quot;tu&quot; CabinClass=&quot;auras sed&quot; FlightNumber=&quot;aequo&quot; Origin=&quot;aeq&quot; Destination=&quot;et &quot; DepartureTime=&quot;incedo regina iovisque&quot; ArrivalTime=&quot;hic vasto rex&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; ProviderCode=&quot;ego q&quot; SupplierCode=&quot;tempe&quot; ParticipantLevel=&quot;ego quae divum&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;insuper altos&quot; AvailabilityDisplayType=&quot;temperat iras&quot; ClassOfService=&quot;ma&quot; ETicketability=&quot;No&quot; Equipment=&quot;alt&quot; MarriageGroup=&quot;3&quot; NumberOfStops=&quot;3&quot; Seamless=&quot;false&quot; ChangeOfPlane=&quot;false&quot; GuaranteedPaymentCarrier=&quot;premere et&quot; HostTokenRef=&quot;iovisque et&quot; ProviderReservationInfoRef=&quot;nubibus ignem disiecitque&quot; PassiveProviderReservationInfoRef=&quot;imposuit regemque&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;m&quot; APISRequirementsRef=&quot;imponet honorem talia&quot; BlackListed=&quot;false&quot; OperationalStatus=&quot;volutans nimborum in&quot; NumberInParty=&quot;99&quot; RailCoachNumber=&quot;mari&quot; BookingDate=&quot;2011-04-18+05:30&quot; FlownSegment=&quot;false&quot; ScheduleChange=&quot;false&quot; BrandIndicator=&quot;ego quae&quot;>
               &lt;com:SegmentRemark Key=&quot;iaculata e nubibus&quot;>omnipotens speluncis&lt;/com:SegmentRemark>
               &lt;air:SponsoredFltInfo SponsoredLNB=&quot;200&quot; NeutralLNB=&quot;200&quot; FltKey=&quot;preme&quot;/>
               &lt;air:CodeshareInfo OperatingCarrier=&quot;ta&quot; OperatingFlightNumber=&quot;terra&quot;>et carcere&lt;/air:CodeshareInfo>
               &lt;air:AirAvailInfo ProviderCode=&quot;dare &quot; HostTokenRef=&quot;vasto rex aeolus&quot;>
                  &lt;air:BookingCodeInfo CabinClass=&quot;arce sceptra&quot; BookingCounts=&quot;luctantis ventos&quot;/>
                  &lt;air:FareTokenInfo FareInfoRef=&quot;bella gero&quot; HostTokenRef=&quot;ac vinclis et&quot;/>
               &lt;/air:AirAvailInfo>
               &lt;air:FlightDetails Key=&quot;imposuit regemque&quot; Origin=&quot;ign&quot; Destination=&quot;et &quot; DepartureTime=&quot;murmure montis&quot; ArrivalTime=&quot;premere et&quot; FlightTime=&quot;100&quot; TravelTime=&quot;100&quot; Distance=&quot;100&quot; Equipment=&quot;rap&quot; OnTimePerformance=&quot;100&quot; OriginTerminal=&quot;rapidum iaculata&quot; DestinationTerminal=&quot;premere et&quot; GroundTime=&quot;100&quot; AutomatedCheckin=&quot;false&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;IgnoreSegment&quot;>
                     &lt;air:FareNote Key=&quot;cum murmure&quot; Precedence=&quot;100&quot; NoteName=&quot;arce sceptra&quot; FareInfoMessageRef=&quot;austris aeoliam venit&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>ni faciat maria&lt;/air:FareNote>
                  &lt;/air:Connection>
                  &lt;air:Meals>HotMeal&lt;/air:Meals>
                  &lt;air:InFlightServices>auras sed&lt;/air:InFlightServices>
               &lt;/air:FlightDetails>
               &lt;air:FlightDetailsRef Key=&quot;iovisque et&quot;/>
               &lt;air:AlternateLocationDistanceRef Key=&quot;flammato secum dea&quot;/>
               &lt;air:Connection ChangeOfPlane=&quot;false&quot; ChangeOfTerminal=&quot;false&quot; ChangeOfAirport=&quot;false&quot; StopOver=&quot;false&quot; MinConnectionTime=&quot;3&quot; Duration=&quot;3&quot; SegmentIndex=&quot;3&quot; FlightDetailsIndex=&quot;3&quot; IncludeStopOverToFareQuote=&quot;IgnoreSegment&quot;>
                  &lt;air:FareNote Key=&quot;et quisquam&quot; Precedence=&quot;100&quot; NoteName=&quot;terras caelumque&quot; FareInfoMessageRef=&quot;rex aeolus antro&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>certo et&lt;/air:FareNote>
               &lt;/air:Connection>
               &lt;com:SellMessage>dare iussus&lt;/com:SellMessage>
               &lt;air:RailCoachDetails RailCoachNumber=&quot;terras caelumque&quot; AvailableRailSeats=&quot;altos imposuit&quot; RailSeatMapAvailability=&quot;false&quot;/>
            &lt;/air:AirSegment>
            &lt;air:FeeInfo BaseAmount=&quot;annos bella gero&quot; Description=&quot;ignem disiecitque&quot; SubCode=&quot;iovisque et soror&quot; Key=&quot;molemque et montis&quot; Amount=&quot;montis insuper&quot; Code=&quot;profundum quippe&quot; FeeToken=&quot;bella gero&quot; PaymentRef=&quot;incedo regina iovisque&quot; BookingTravelerRef=&quot;iovis rapidum&quot; PassengerTypeCode=&quot;et pr&quot; Text=&quot;faciat maria&quot; ProviderCode=&quot;trans&quot; SupplierCode=&quot;claus&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:TaxInfoRef Key=&quot;corde volutans nimborum&quot;/>
               &lt;com:IncludedInBase Amount=&quot;et soror et&quot;/>
            &lt;/air:FeeInfo>
            &lt;air:TaxInfo Key=&quot;quae divum&quot; Category=&quot;caelumque profundum quippe&quot; CarrierDefinedCategory=&quot;sceptra tenens mollitque&quot; SegmentRef=&quot;volutans nimborum in&quot; FlightDetailsRef=&quot;venit hic&quot; CouponRef=&quot;et laxas&quot; Amount=&quot;ferant rapidi&quot; OriginAirport=&quot;arc&quot; DestinationAirport=&quot;ign&quot; CountryCode=&quot;pater omnipotens&quot; FareInfoRef=&quot;sedet aeolus&quot; TaxExempted=&quot;true&quot; ProviderCode=&quot;scire&quot; SupplierCode=&quot;secum&quot; Text=&quot;et carcere&quot;>
               &lt;com:TaxDetail Amount=&quot;iaculata e&quot; OriginAirport=&quot;div&quot; DestinationAirport=&quot;ins&quot; CountryCode=&quot;imperio premit&quot; FareInfoRef=&quot;hic vasto rex&quot;/>
               &lt;com:IncludedInBase Amount=&quot;murmure montis&quot;/>
            &lt;/air:TaxInfo>
            &lt;com:HostToken Host=&quot;illi &quot; Key=&quot;anySimpleType&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>profundum quippe&lt;/com:HostToken>
         &lt;/air:TCRRefundBundle>
         &lt;air:AirRefundModifiers RefundDate=&quot;feta furentibus austris&quot; AccountCode=&quot;murmure montis&quot; TicketDesignator=&quot;numen iunonis adorat&quot;/>
         &lt;com:Commission Key=&quot;ventos tempestatesque sonoras&quot; Level=&quot;Penalty&quot; Type=&quot;Flat&quot; Modifier=&quot;StandardPlusSupplementaryPercent&quot; Amount=&quot;quippe ferant&quot; Value=&quot;sonoras imperio&quot; Percentage=&quot;illi indignantes&quot; BookingTravelerRef=&quot;ventos tempestatesque sonoras&quot; CommissionOverride=&quot;false&quot;/>
         &lt;com:FormOfPayment Key=&quot;molemque et montis&quot; Type=&quot;mollitque animos et&quot; FulfillmentType=&quot;tenens mollitque&quot; FulfillmentLocation=&quot;circum claustra&quot; FulfillmentIDType=&quot;Euro Cheque Card&quot; FulfillmentIDNumber=&quot;magno cum murmure&quot; IsAgentType=&quot;false&quot; AgentText=&quot;iaculata e&quot; ReuseFOP=&quot;sciret dare&quot; ExternalReference=&quot;adorat praeterea aut&quot; Reusable=&quot;false&quot; ProfileID=&quot;aeolus arce sceptra&quot; ProfileKey=&quot;profundum quippe ferant&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
            &lt;com:CreditCard Type=&quot;ig&quot; Number=&quot;dare iussusdare iussus&quot; ExpDate=&quot;2014-03&quot; Name=&quot;talia flammato&quot; CVV=&quot;aura&quot; ApprovalCode=&quot;ventis illum exs&quot; ExtendedPayment=&quot;rapidum iaculata&quot; CustomerReference=&quot;claustra fremunt&quot; AcceptanceOverride=&quot;false&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;caelumque profundum quippe&quot; BankCountryCode=&quot;te&quot; BankStateCode=&quot;rates &quot; Enett=&quot;false&quot; ProfileID=&quot;laxas sciret&quot; Key=&quot;gente tot&quot;>
               &lt;com:PhoneNumber Key=&quot;speluncis abdidit&quot; Type=&quot;Email&quot; Location=&quot;aequora ve&quot; CountryCode=&quot;exspi&quot; AreaCode=&quot;dare iussu&quot; Number=&quot;aut supplex&quot; Extension=&quot;iunonis ad&quot; Text=&quot;aris imponet honorem&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;venit hic&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;numen iunonis adorat&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>disiecitque rates&lt;/com:AddressName>
                  &lt;com:Street>iovisque et&lt;/com:Street>
                  &lt;com:City>ego quae&lt;/com:City>
                  &lt;com:State>flammas turbine&lt;/com:State>
                  &lt;com:PostalCode>ventos tempesta&lt;/com:PostalCode>
                  &lt;com:Country>ra&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;et temperat&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;fa&quot; Number=&quot;sedet aeolussedet aeolus&quot; ExpDate=&quot;2014-09&quot; Name=&quot;austris aeoliam&quot; CVV=&quot;divu&quot; ApprovalCode=&quot;et quisquam&quot; IssueNumber=&quot;et coniu&quot; ProfileID=&quot;tempestatesque sonoras&quot; Key=&quot;soror et&quot;>
               &lt;com:PhoneNumber Key=&quot;murmure montis&quot; Type=&quot;Agency&quot; Location=&quot;nubibus ig&quot; CountryCode=&quot;rapid&quot; AreaCode=&quot;et carcere&quot; Number=&quot;austris aeoliam&quot; Extension=&quot;claustra f&quot; Text=&quot;circum claustra&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;gero et&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;rapidum iaculata e&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>laxas sciret dare&lt;/com:AddressName>
                  &lt;com:Street>auras sed&lt;/com:Street>
                  &lt;com:City>regina iovisque et&lt;/com:City>
                  &lt;com:State>pater omnipotens speluncis&lt;/com:State>
                  &lt;com:PostalCode>corripuit scopu&lt;/com:PostalCode>
                  &lt;com:Country>pa&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;infixit acuto&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;montis circum claustra&quot; Amount=&quot;iaculata e&quot; DiscountAmount=&quot;metuens molemque&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2018-11-23&quot; NotValidAfter=&quot;2019-05-22&quot;/>
            &lt;com:TicketNumber>evertitque ae&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;et quisquam&quot; RoutingNumber=&quot;flammas turbine&quot; AccountNumber=&quot;transfixo pectore flammas&quot; CheckNumber=&quot;cum gente&quot;/>
            &lt;com:Requisition Number=&quot;frenat illi&quot; Category=&quot;Other&quot; Type=&quot;Cash&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;ir&quot; CreditCardNumber=&quot;disiecitque rates&quot; ExpDate=&quot;2003-09&quot; Text=&quot;aeolus arce sceptra&quot; Category=&quot;imperio premit ac&quot; AcceptanceOverride=&quot;false&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;furentibus austris&quot; AgencyBillingNumber=&quot;cum murmure&quot; AgencyBillingPassword=&quot;et carcere&quot;/>
            &lt;com:UnitedNations Number=&quot;cum gente&quot;/>
            &lt;com:DirectPayment Text=&quot;terras caelumque&quot;/>
            &lt;com:AgentVoucher Number=&quot;aeolus arce sceptra&quot;/>
            &lt;com:PaymentAdvice Type=&quot;son&quot; DocumentNumber=&quot;feta furentibus&quot; IssueDate=&quot;2006-02-06&quot; IssueCity=&quot;ira&quot; OriginalFOP=&quot;fremunt celsa&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;dea corde&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;aris imponet honorem&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;aeolus antro&quot; BSPPassword=&quot;tempestatesque sonoras imperio&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;sciret dare iussus&quot; ARCPassword=&quot;vasto rex&quot;/>
         &lt;/com:FormOfPayment>
      &lt;/air:AirRefundReq>
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
