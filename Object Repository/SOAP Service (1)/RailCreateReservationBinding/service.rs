<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>172f4386-2e98-473b-b16a-9b489323b624</elementGuidId>
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
      <value>http://localhost:8080/kestrel/RailService</value>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot; xmlns:rail=&quot;http://www.travelport.com/schema/rail_v51_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;univ:RailCreateReservationReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; RuleName=&quot;regemque d&quot; UniversalRecordLocatorCode=&quot;fremunt &quot; ProviderLocatorCode=&quot;ferant r&quot; ProviderCode=&quot;sceptra tenens&quot; CustomerNumber=&quot;turbine corripuit scopuloque&quot; Version=&quot;100&quot; BookingActionType=&quot;flammato secum dea&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;profundum quippe ferant&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;nubib&quot; ProviderCode=&quot;ac vi&quot; AgentID=&quot;rapidum iaculata e&quot;/>
         &lt;com:TerminalSessionInfo>caelumque profundum&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;circu&quot; PseudoCityCode=&quot;coniunx un&quot;/>
         &lt;com:LinkedUniversalRecord LocatorCode=&quot;arce sce&quot; Key=&quot;circum claustra fremunt&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
         &lt;com:BookingTraveler Key=&quot;et montis&quot; TravelerType=&quot;feta &quot; Age=&quot;100&quot; VIP=&quot;false&quot; DOB=&quot;2000-07-28-06:00&quot; Gender=&quot;pe&quot; Nationality=&quot;an&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; NameNumber=&quot;rates evertitque aequora&quot;>
            &lt;com:BookingTravelerName Prefix=&quot;volutans nimborum in&quot; First=&quot;faciat maria&quot; Middle=&quot;et quisquam&quot; Last=&quot;et soror&quot; Suffix=&quot;annos bella gero&quot;/>
            &lt;com:DeliveryInfo Type=&quot;ventos tempestatesque sonoras&quot; SignatureRequired=&quot;regina iov&quot; TrackingNumber=&quot;rapidum iaculata&quot;>
               &lt;com:ShippingAddress Key=&quot;insuper altos&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>in patriam&lt;/com:AddressName>
                  &lt;com:Street>abdidit atris hoc&lt;/com:Street>
                  &lt;com:City>coniunx una cum&lt;/com:City>
                  &lt;com:State>ipsa iovis&lt;/com:State>
                  &lt;com:PostalCode>frenat illi ind&lt;/com:PostalCode>
                  &lt;com:Country>ni&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;mollitque animos&quot;/>
               &lt;/com:ShippingAddress>
               &lt;com:PhoneNumber Key=&quot;montis insuper altos&quot; Type=&quot;Home&quot; Location=&quot;ferant rap&quot; CountryCode=&quot;nimbo&quot; AreaCode=&quot;vasto rex&quot; Number=&quot;flammato secum&quot; Extension=&quot;tempestate&quot; Text=&quot;tenens mollitque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;iras ni&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:Email Key=&quot;nubibus ignem&quot; Type=&quot;austris aeoliam venit&quot; Comment=&quot;qui foedere&quot; EmailID=&quot;atris hoc metuens&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;animos et temperat&quot;/>
               &lt;/com:Email>
               &lt;com:GeneralRemark Key=&quot;sonoras imperio&quot; Category=&quot;patriam loca&quot; TypeInGds=&quot;montis circum&quot; SupplierType=&quot;Other&quot; ProviderReservationInfoRef=&quot;tempestatesque sonoras imperio&quot; ProviderCode=&quot;scept&quot; SupplierCode=&quot;vasto&quot; Direction=&quot;Outgoing&quot; CreateDate=&quot;2016-04-18T06:24:51-06:00&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:RemarkData>auras sed&lt;/com:RemarkData>
                  &lt;com:BookingTravelerRef>aequora ventis illum&lt;/com:BookingTravelerRef>
               &lt;/com:GeneralRemark>
               &lt;com:ProviderReservationInfoRef Key=&quot;aequora ventis&quot;/>
            &lt;/com:DeliveryInfo>
            &lt;com:PhoneNumber Key=&quot;et coniunx&quot; Type=&quot;Fax&quot; Location=&quot;imperio pr&quot; CountryCode=&quot;arce &quot; AreaCode=&quot;ni faciat&quot; Number=&quot;circum claustra fremunt&quot; Extension=&quot;corde volu&quot; Text=&quot;insuper altos&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;magno cum&quot;/>
            &lt;/com:PhoneNumber>
            &lt;com:Email Key=&quot;gero et&quot; Type=&quot;iunonis adorat praeterea&quot; Comment=&quot;premere et&quot; EmailID=&quot;iovisque et&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;imposuit regemque&quot;/>
            &lt;/com:Email>
            &lt;com:LoyaltyCard Key=&quot;rates evertitque&quot; SupplierCode=&quot;su&quot; AllianceLevel=&quot;ipsa iovis&quot; MembershipProgram=&quot;volutans nimborum in&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot; CardNumber=&quot;foedere certo&quot; Status=&quot;sciret dare iussus&quot; MembershipStatus=&quot;rapidi secum verrantque&quot; FreeText=&quot;montis circum&quot; SupplierType=&quot;Other&quot; Level=&quot;premere et laxas&quot; PriorityCode=&quot;talia flammato&quot; VendorLocationRef=&quot;terras caelumque&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;false&quot; ReservationLevel=&quot;true&quot;>
                  &lt;com:OperatedBy>dare iussus&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;vasto rex aeolus&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;arce sceptra&quot; Code=&quot;luctanti&quot; Description=&quot;bella gero&quot; Number=&quot;ac vinclis et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;/>
            &lt;com:SSR Key=&quot;ignem disiecitque rates&quot; SegmentRef=&quot;et premere&quot; PassiveSegmentRef=&quot;murmure montis&quot; ProviderReservationInfoRef=&quot;premere et&quot; Type=&quot;rapi&quot; Status=&quot;rapidum iaculata&quot; FreeText=&quot;premere et&quot; Carrier=&quot;in&quot; CarrierSpecificText=&quot;maria ac terras&quot; Description=&quot;rates evertitque&quot; ProviderDefinedType=&quot;iovis rapidum&quot; SSRRuleRef=&quot;indignantes magno&quot; URL=&quot;http://www.sample.com/ni/rapidum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; ProfileID=&quot;iovisque et&quot; ProfileSecureFlightDocKey=&quot;flammato secum dea&quot;/>
            &lt;com:NameRemark Key=&quot;hic vasto rex&quot; Category=&quot;aeoliam venit&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
               &lt;com:RemarkData>profundum quippe&lt;/com:RemarkData>
               &lt;com:ProviderReservationInfoRef Key=&quot;premit ac vinclis&quot;/>
            &lt;/com:NameRemark>
            &lt;com:AirSeatAssignment Key=&quot;luctantis ventos&quot; Status=&quot;lu&quot; Seat=&quot;nubibus ignem&quot; SeatTypeCode=&quot;et s&quot; SegmentRef=&quot;annos bella gero&quot; FlightDetailsRef=&quot;ignem disiecitque&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; RailCoachNumber=&quot;molemque et montis&quot;/>
            &lt;com:RailSeatAssignment Key=&quot;montis insuper&quot; Status=&quot;pr&quot; Seat=&quot;bella gero&quot; RailSegmentRef=&quot;incedo regina iovisque&quot; CoachNumber=&quot;iovis rapidum&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
               &lt;com:Characteristic SeatType=&quot;faciat maria&quot; SeatDescription=&quot;transfixo pectore flammas&quot; SeatValue=&quot;claustra fremunt&quot; SeatValueDescription=&quot;una cum&quot;/>
            &lt;/com:RailSeatAssignment>
            &lt;com:EmergencyInfo>corde volutans nimborum&lt;/com:EmergencyInfo>
            &lt;com:Address Key=&quot;et soror et&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
               &lt;com:AddressName>caelumque profundum quippe&lt;/com:AddressName>
               &lt;com:Street>sceptra tenens mollitque&lt;/com:Street>
               &lt;com:City>volutans nimborum in&lt;/com:City>
               &lt;com:State>venit hic&lt;/com:State>
               &lt;com:PostalCode>et laxas&lt;/com:PostalCode>
               &lt;com:Country>fe&lt;/com:Country>
               &lt;com:ProviderReservationInfoRef Key=&quot;arce sceptra tenens&quot;/>
            &lt;/com:Address>
            &lt;com:DriversLicense Key=&quot;ignem disiecitque rates&quot; LicenseNumber=&quot;pater omnipotens&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:AppliedProfile Key=&quot;scopuloque infixit acuto&quot; TravelerID=&quot;mollitque animos&quot; TravelerName=&quot;bella gero&quot; AccountID=&quot;ast ego&quot; AccountName=&quot;nubibus ignem&quot; ImmediateParentID=&quot;aut supplex&quot; ImmediateParentName=&quot;ni faciat&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:CustomizedNameData Key=&quot;imposuit regemque dedit&quot; ProviderReservationInfoRef=&quot;claustra fremunt celsa&quot;>pater omnipotens&lt;/com:CustomizedNameData>
            &lt;com:TravelComplianceData Key=&quot;quippe ferant rapidi&quot; AirSegmentRef=&quot;regina iovisque&quot; PassiveSegmentRef=&quot;exspirantem transfixo&quot; RailSegmentRef=&quot;honorem talia flammato&quot; ReservationLocatorRef=&quot;mollitqu&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
               &lt;com:PolicyCompliance InPolicy=&quot;true&quot; PolicyToken=&quot;caelumque profundum quippe&quot;/>
               &lt;com:ContractCompliance InContract=&quot;false&quot; ContractToken=&quot;sonoras imperio&quot;/>
               &lt;com:PreferredSupplier Preferred=&quot;true&quot; ProfileType=&quot;Account&quot;/>
            &lt;/com:TravelComplianceData>
            &lt;com:TravelInfo TripName=&quot;ventos tempestatesque sonoras&quot; TravelPurpose=&quot;molemque et montis&quot;/>
         &lt;/com:BookingTraveler>
         &lt;com:OSI Key=&quot;mollitque animos et&quot; Carrier=&quot;te&quot; Code=&quot;circ&quot; Text=&quot;circum claustra fremunt&quot; ProviderReservationInfoRef=&quot;acuto ast&quot; ProviderCode=&quot;honor&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
         &lt;com:AccountingRemark Key=&quot;premere et laxas&quot; Category=&quot;flammas turbin&quot; TypeInGds=&quot;ipsa iovis&quot; ProviderReservationInfoRef=&quot;sedet aeolus&quot; ProviderCode=&quot;frena&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>in patriam loca&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>talia flammato&lt;/com:BookingTravelerRef>
         &lt;/com:AccountingRemark>
         &lt;com:GeneralRemark Key=&quot;auras sed&quot; Category=&quot;ventis illum exspira&quot; TypeInGds=&quot;rapidum iaculata&quot; SupplierType=&quot;Air&quot; ProviderReservationInfoRef=&quot;habenas ipsa&quot; ProviderCode=&quot;caelu&quot; SupplierCode=&quot;tenen&quot; Direction=&quot;Outgoing&quot; CreateDate=&quot;2009-03-06T08:09:33-07:00&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>speluncis abdidit&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>patriam loca feta&lt;/com:BookingTravelerRef>
         &lt;/com:GeneralRemark>
         &lt;com:XMLRemark Key=&quot;molemque et&quot; Category=&quot;animos et&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>montis circum&lt;/com:XMLRemark>
         &lt;com:UnassociatedRemark ProviderReservationInfoRef=&quot;ventos tempestatesque&quot; ProviderCode=&quot;et te&quot; Key=&quot;foedere certo&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>hic vasto rex&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>per auras sed&lt;/com:BookingTravelerRef>
         &lt;/com:UnassociatedRemark>
         &lt;com:Postscript ProviderReservationInfoRef=&quot;vinclis et&quot; ProviderCode=&quot;monti&quot; Key=&quot;acuto ast ego&quot;>secum dea&lt;/com:Postscript>
         &lt;com:PassiveInfo ProviderCode=&quot;iras ni&quot; ProviderLocatorCode=&quot;et quisquam&quot; SupplierCode=&quot;carcere frenat illi&quot; SupplierLocatorCode=&quot;iras ni&quot;>
            &lt;com:TicketNumber>iras ni&lt;/com:TicketNumber>
            &lt;com:ConfirmationNumber>tempestatesque sonoras imperio&lt;/com:ConfirmationNumber>
            &lt;com:Commission Key=&quot;austris aeoliam&quot; Level=&quot;Fare&quot; Type=&quot;Flat&quot; Modifier=&quot;FareAmount&quot; Amount=&quot;montis circum&quot; Value=&quot;aeoliam venit h&quot; Percentage=&quot;talia flammato&quot; BookingTravelerRef=&quot;caelumque profundum&quot; CommissionOverride=&quot;false&quot;/>
         &lt;/com:PassiveInfo>
         &lt;com:ContinuityCheckOverride Key=&quot;aeoliam venit&quot;>nubibus ignem&lt;/com:ContinuityCheckOverride>
         &lt;com:AgencyContactInfo Key=&quot;rapidi secum&quot;>
            &lt;com:PhoneNumber Key=&quot;et carcere frenat&quot; Type=&quot;Fax&quot; Location=&quot;caelumque &quot; CountryCode=&quot;patri&quot; AreaCode=&quot;insuper al&quot; Number=&quot;et carcere&quot; Extension=&quot;laxas scir&quot; Text=&quot;infixit acuto&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;magno cum murmure&quot;/>
            &lt;/com:PhoneNumber>
         &lt;/com:AgencyContactInfo>
         &lt;com:CustomerID ProviderReservationInfoRef=&quot;annos bella&quot; ProviderCode=&quot;soror&quot; Key=&quot;annos bella&quot;>qui foedere&lt;/com:CustomerID>
         &lt;com:FileFinishingInfo>
            &lt;com:ShopInformation CabinShopped=&quot;gero et&quot; CabinSelected=&quot;speluncis abdidit atris&quot; LowestFareOffered=&quot;per auras&quot;>
               &lt;com:SearchRequest Origin=&quot;alt&quot; Destination=&quot;aeo&quot; DepartureTime=&quot;ventis illum&quot; ClassOfService=&quot;al&quot;/>
               &lt;com:FlightsOffered Origin=&quot;ven&quot; Destination=&quot;ius&quot; DepartureTime=&quot;aeolus antro&quot; TravelOrder=&quot;3&quot; Carrier=&quot;ig&quot; FlightNumber=&quot;gente&quot; ClassOfService=&quot;im&quot; StopOver=&quot;false&quot; Connection=&quot;false&quot;/>
            &lt;/com:ShopInformation>
            &lt;com:PolicyInformation Type=&quot;Air&quot; Name=&quot;anySimpleType&quot; OutOfPolicy=&quot;false&quot; SegmentRef=&quot;altos imposuit&quot;>
               &lt;com:ReasonCode>
                  &lt;com:OutOfPolicy>scopuloque infixit&lt;/com:OutOfPolicy>
                  &lt;com:PurposeOfTrip>ni faciat maria&lt;/com:PurposeOfTrip>
                  &lt;com:Remark Key=&quot;regemque dedit qui&quot;>speluncis abdidit&lt;/com:Remark>
               &lt;/com:ReasonCode>
            &lt;/com:PolicyInformation>
            &lt;com:AccountInformation AccountName=&quot;aeolus arce sceptra&quot;>
               &lt;com:Address Key=&quot;imperio premit ac&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>flammato secum dea&lt;/com:AddressName>
                  &lt;com:Street>faciat maria&lt;/com:Street>
                  &lt;com:City>ignem disiecitque&lt;/com:City>
                  &lt;com:State>illum exspirantem&lt;/com:State>
                  &lt;com:PostalCode>et premere&lt;/com:PostalCode>
                  &lt;com:Country>vo&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;feta furentibus&quot;/>
               &lt;/com:Address>
               &lt;com:PhoneNumber Key=&quot;et temperat&quot; Type=&quot;Other&quot; Location=&quot;imposuit r&quot; CountryCode=&quot;laxas&quot; AreaCode=&quot;maria ac t&quot; Number=&quot;furentibus austris aeoliam&quot; Extension=&quot;corde volu&quot; Text=&quot;disiecitque rates evertitque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;aut supplex&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AccountInformation>
            &lt;com:AgencyInformation>
               &lt;com:Address Key=&quot;atris hoc&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>sceptra tenens mollitque&lt;/com:AddressName>
                  &lt;com:Street>fremunt celsa&lt;/com:Street>
                  &lt;com:City>aut supplex&lt;/com:City>
                  &lt;com:State>et montis insuper&lt;/com:State>
                  &lt;com:PostalCode>sedet aeolus&lt;/com:PostalCode>
                  &lt;com:Country>ir&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;coniunx una cum&quot;/>
               &lt;/com:Address>
               &lt;com:Email Key=&quot;quae divum&quot; Type=&quot;circum claustra fremunt&quot; Comment=&quot;et montis&quot; EmailID=&quot;speluncis abdidit&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;disiecitque rates&quot;/>
               &lt;/com:Email>
               &lt;com:PhoneNumber Key=&quot;dea corde volutans&quot; Type=&quot;Home&quot; Location=&quot;patriam lo&quot; CountryCode=&quot;celsa&quot; AreaCode=&quot;exspirante&quot; Number=&quot;coniunx una&quot; Extension=&quot;aequora ve&quot; Text=&quot;disiecitque rates&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;atris hoc&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AgencyInformation>
            &lt;com:TravelerInformation HomeAirport=&quot;abd&quot; VisaExpirationDate=&quot;2002-05-21&quot; BookingTravelerRef=&quot;et premere et&quot;>
               &lt;com:EmergencyContact Name=&quot;anySimpleType&quot; Relationship=&quot;anySimpleType&quot;>
                  &lt;com:PhoneNumber Key=&quot;certo et premere&quot; Type=&quot;Reservations&quot; Location=&quot;et montis&quot; CountryCode=&quot;imper&quot; AreaCode=&quot;secum dea&quot; Number=&quot;regemque dedit&quot; Extension=&quot;corde volu&quot; Text=&quot;vinclis et&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;circum claustra fremunt&quot;/>
                  &lt;/com:PhoneNumber>
               &lt;/com:EmergencyContact>
            &lt;/com:TravelerInformation>
            &lt;com:CustomProfileInformation/>
         &lt;/com:FileFinishingInfo>
         &lt;com:CommissionRemark Key=&quot;pectore flammas&quot; ProviderReservationInfoRef=&quot;quae divum&quot; ProviderCode=&quot;secum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
            &lt;com:ProviderReservationLevel Amount=&quot;imperio premit&quot; Percentage=&quot;quae divum&quot; CommissionCap=&quot;ni faciat&quot;/>
            &lt;com:PassengerTypeLevel TravelerType=&quot;caelu&quot; Amount=&quot;aeolus arce&quot; Percentage=&quot;illi indignantes&quot; CommissionCap=&quot;ac vinclis&quot;/>
         &lt;/com:CommissionRemark>
         &lt;com:ConsolidatorRemark Key=&quot;habenas ipsa&quot; ProviderReservationInfoRef=&quot;sed pater omnipotens&quot; ProviderCode=&quot;iovis&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:PseudoCityCode>iussus hab&lt;/com:PseudoCityCode>
         &lt;/com:ConsolidatorRemark>
         &lt;com:InvoiceRemark ProviderReservationInfoRef=&quot;ipsa iovis&quot; ProviderCode=&quot;dedit&quot; Key=&quot;ipsa iovis rapidum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>auras sed&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>tenens mollitque&lt;/com:BookingTravelerRef>
            &lt;com:AirSegmentRef Key=&quot;exspirantem transfixo&quot;/>
            &lt;com:HotelReservationRef LocatorCode=&quot;exspiran&quot;/>
            &lt;com:VehicleReservationRef LocatorCode=&quot;vinclis &quot;/>
            &lt;com:PassiveSegmentRef Key=&quot;divum incedo regina&quot;/>
         &lt;/com:InvoiceRemark>
         &lt;com:SSR Key=&quot;furentibus austris&quot; SegmentRef=&quot;volutans nimborum&quot; PassiveSegmentRef=&quot;claustra fremunt celsa&quot; ProviderReservationInfoRef=&quot;et laxas sciret&quot; Type=&quot;habe&quot; Status=&quot;gente tot annos&quot; FreeText=&quot;premere et&quot; Carrier=&quot;et&quot; CarrierSpecificText=&quot;ego quae&quot; Description=&quot;dea corde volutans&quot; ProviderDefinedType=&quot;celsa sedet&quot; SSRRuleRef=&quot;flammato secum&quot; URL=&quot;http://www.company.com/sed/pater&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot; ProfileID=&quot;rapidum iaculata e&quot; ProfileSecureFlightDocKey=&quot;e nubibus&quot;/>
         &lt;com:EmailNotification Recipients=&quot;Default&quot;>
            &lt;com:EmailRef>ipsa iovis&lt;/com:EmailRef>
         &lt;/com:EmailNotification>
         &lt;com:QueuePlace>
            &lt;com:PseudoCityCode>corde volu&lt;/com:PseudoCityCode>
            &lt;com:QueueSelector Queue=&quot;iaculata e&quot; Category=&quot;carcere frenat illi&quot; DateRange=&quot;aut supplex aris&quot;/>
         &lt;/com:QueuePlace>
         &lt;com:FormOfPayment Key=&quot;molemque et&quot; Type=&quot;feta furentibus&quot; FulfillmentType=&quot;bella gero&quot; FulfillmentLocation=&quot;ac terras&quot; FulfillmentIDType=&quot;Collection Reference&quot; FulfillmentIDNumber=&quot;rapidum iaculata e&quot; IsAgentType=&quot;false&quot; AgentText=&quot;quippe ferant&quot; ReuseFOP=&quot;austris aeoliam&quot; ExternalReference=&quot;infixit acuto&quot; Reusable=&quot;false&quot; ProfileID=&quot;animos et&quot; ProfileKey=&quot;aequora ventis illum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:CreditCard Type=&quot;pa&quot; Number=&quot;supplex aris imponet&quot; ExpDate=&quot;2013-02&quot; Name=&quot;verrantque per&quot; CVV=&quot;una &quot; ApprovalCode=&quot;sonoras imperio&quot; ExtendedPayment=&quot;patriam loca feta&quot; CustomerReference=&quot;gero et&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;e nubibus&quot; BankCountryCode=&quot;ae&quot; BankStateCode=&quot;transf&quot; Enett=&quot;false&quot; ProfileID=&quot;iovis rapidum iaculata&quot; Key=&quot;venit hic&quot;>
               &lt;com:PhoneNumber Key=&quot;flammas turbine corripuit&quot; Type=&quot;Reservations&quot; Location=&quot;ni faciat&quot; CountryCode=&quot;abdid&quot; AreaCode=&quot;evertitque&quot; Number=&quot;illi indignantes&quot; Extension=&quot;terras cae&quot; Text=&quot;rates evertitque&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;circum claustra&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;aeolus antro luctantis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>certo et&lt;/com:AddressName>
                  &lt;com:Street>antro luctantis&lt;/com:Street>
                  &lt;com:City>terras caelumque&lt;/com:City>
                  &lt;com:State>vasto rex&lt;/com:State>
                  &lt;com:PostalCode>honorem talia f&lt;/com:PostalCode>
                  &lt;com:Country>ci&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;verrantque per&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;nu&quot; Number=&quot;certo etcerto et&quot; ExpDate=&quot;2015-01&quot; Name=&quot;tenens mollitque&quot; CVV=&quot;acut&quot; ApprovalCode=&quot;et laxas&quot; IssueNumber=&quot;montis i&quot; ProfileID=&quot;quae divum incedo&quot; Key=&quot;talia flammato&quot;>
               &lt;com:PhoneNumber Key=&quot;aeoliam venit&quot; Type=&quot;Business&quot; Location=&quot;premit ac&quot; CountryCode=&quot;et co&quot; AreaCode=&quot;insuper al&quot; Number=&quot;annos bella gero&quot; Extension=&quot;rapidum ia&quot; Text=&quot;quisquam numen iunonis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;premit ac&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;iovisque et&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>omnipotens speluncis abdidit&lt;/com:AddressName>
                  &lt;com:Street>et montis insuper&lt;/com:Street>
                  &lt;com:City>magno cum&lt;/com:City>
                  &lt;com:State>infixit acuto ast&lt;/com:State>
                  &lt;com:PostalCode>tot annos&lt;/com:PostalCode>
                  &lt;com:Country>ni&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;aeolus arce sceptra&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;bella gero et&quot; Amount=&quot;hoc metuens&quot; DiscountAmount=&quot;luctantis ventos tempestatesque&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2019-01-03&quot; NotValidAfter=&quot;2019-08-13-06:00&quot;/>
            &lt;com:TicketNumber>imperio premi&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;exspirantem transfixo&quot; RoutingNumber=&quot;sceptra tenens&quot; AccountNumber=&quot;aequora ventis&quot; CheckNumber=&quot;regina iovisque&quot;/>
            &lt;com:Requisition Number=&quot;sed pater&quot; Category=&quot;Government&quot; Type=&quot;Cash&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;ge&quot; CreditCardNumber=&quot;caelumque profundum&quot; ExpDate=&quot;2003-09-06:00&quot; Text=&quot;iussus habenas ipsa&quot; Category=&quot;montis insuper&quot; AcceptanceOverride=&quot;true&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;claustra fremunt&quot; AgencyBillingNumber=&quot;ego quae&quot; AgencyBillingPassword=&quot;imponet honorem&quot;/>
            &lt;com:UnitedNations Number=&quot;nimborum in&quot;/>
            &lt;com:DirectPayment Text=&quot;rapidum iaculata&quot;/>
            &lt;com:AgentVoucher Number=&quot;ventis illum exspirantem&quot;/>
            &lt;com:PaymentAdvice Type=&quot;sed&quot; DocumentNumber=&quot;molemque et&quot; IssueDate=&quot;2018-08-28-06:00&quot; IssueCity=&quot;reg&quot; OriginalFOP=&quot;magno cum murmure&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;supplex aris imponet&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;animos et&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;illum exspirantem&quot; BSPPassword=&quot;cum murmure&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;dare iussus&quot; ARCPassword=&quot;rex aeolus antro&quot;/>
         &lt;/com:FormOfPayment>
         &lt;rail:RailPricingSolution Key=&quot;circum claustra&quot; OfferId=&quot;100&quot; TotalPrice=&quot;sceptra tenens&quot; BasePrice=&quot;tenens mollitque&quot; ApproximateTotalPrice=&quot;altos imposuit regemque&quot; ApproximateBasePrice=&quot;aeolus arce&quot; EquivalentBasePrice=&quot;coniunx una&quot; Taxes=&quot;verrantque per&quot; Fees=&quot;rapidi secum verrantque&quot; Services=&quot;bella gero&quot; ApproximateTaxes=&quot;iovis rapidum&quot; ApproximateFees=&quot;honorem talia&quot; ProviderCode=&quot;rex a&quot; SupplierCode=&quot;quae &quot; HostTokenRef=&quot;per auras&quot; Reference=&quot;iaculata e&quot;>
            &lt;rail:RailJourney Key=&quot;corripuit scopuloque infixit&quot; Origin=&quot;acu&quot; Destination=&quot;eve&quot; DepartureTime=&quot;sonoras imperio&quot; ArrivalTime=&quot;fremunt celsa&quot; OriginStationName=&quot;et carcere frenat&quot; DestinationStationName=&quot;verrantque per&quot; RailLocOrigin=&quot;evertitq&quot; RailLocDestination=&quot;sedet ae&quot; RouteDescription=&quot;et laxas sciret&quot; JourneyDirection=&quot;Outward&quot; JourneyDuration=&quot;100&quot; TotalPrice=&quot;infixit acuto&quot; BasePrice=&quot;temperat iras&quot; ApproximateTotalPrice=&quot;gero et&quot; ApproximateBasePrice=&quot;illi indignantes&quot; EquivalentBasePrice=&quot;foedere certo&quot; Taxes=&quot;per auras sed&quot; Fees=&quot;in patriam&quot; Services=&quot;ventis illum&quot; ApproximateTaxes=&quot;et coniunx una&quot; ApproximateFees=&quot;fremunt celsa sedet&quot; ProviderCode=&quot;dedit&quot; SupplierCode=&quot;fremu&quot; Status=&quot;tot annos bella&quot; ProviderReservationInfoRef=&quot;ego quae&quot; PassiveProviderReservationInfoRef=&quot;ac terras&quot; TravelOrder=&quot;100&quot; RouteReference=&quot;et soror et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot; Operation=&quot;coniunx una&quot;>
               &lt;rail:RailSegment Key=&quot;incedo regina&quot; Status=&quot;ferant rapidi&quot; Passive=&quot;true&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; TrainNumber=&quot;quae div&quot; Origin=&quot;rap&quot; Destination=&quot;exs&quot; DepartureTime=&quot;venit hic vasto&quot; ArrivalTime=&quot;frenat illi&quot; OriginStationName=&quot;cum gente&quot; DestinationStationName=&quot;adorat praeterea&quot; RailLocOrigin=&quot;corripui&quot; RailLocDestination=&quot;terras c&quot; TrainType=&quot;ac vinclis&quot; TrainTypeCode=&quot;montis c&quot; TransportMode=&quot;Sedan Chair&quot; SeatAssignable=&quot;true&quot; TransportCode=&quot;celsa sedet&quot; ReservationRequired=&quot;true&quot; TravelTime=&quot;100&quot; HostTokenRef=&quot;et quisquam&quot; CabinClass=&quot;nimborum in patriam&quot; ClassCode=&quot;qui foed&quot;>
                  &lt;com:SegmentRemark Key=&quot;talia flammato secum&quot;>talia flammato secum&lt;/com:SegmentRemark>
                  &lt;rail:RailSegmentInfo Category=&quot;feta furentibus&quot; Type=&quot;Vendor&quot;>patriam loca&lt;/rail:RailSegmentInfo>
                  &lt;rail:OperatingCompany Code=&quot;circum claustra&quot; Name=&quot;quippe ferant rapidi&quot;>iovisque et soror&lt;/rail:OperatingCompany>
                  &lt;rail:RailAvailInfo ClassCode=&quot;aeoliam &quot; Quantity=&quot;100&quot; CabinClass=&quot;illi indignantes magno&quot;/>
                  &lt;rail:FulFillmentType>e nubibus&lt;/rail:FulFillmentType>
               &lt;/rail:RailSegment>
               &lt;rail:RailSegmentRef Key=&quot;adorat praeterea&quot;/>
               &lt;rail:JourneyRemark Category=&quot;quae divum&quot;>iussus habenas&lt;/rail:JourneyRemark>
               &lt;com:HostToken Host=&quot;et la&quot; Key=&quot;anySimpleType&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>murmure montis circum&lt;/com:HostToken>
            &lt;/rail:RailJourney>
            &lt;rail:RailJourneyRef Key=&quot;aequora ventis&quot;/>
            &lt;rail:RailPricingInfo Key=&quot;ventos tempestatesque sonoras&quot; ExchangeAmount=&quot;secum verrantque per&quot; ApproximateExchangeAmount=&quot;una cum&quot; TotalPrice=&quot;bella gero&quot; BasePrice=&quot;evertitque aequora&quot; ApproximateTotalPrice=&quot;premere et&quot; ApproximateBasePrice=&quot;sed pater&quot; EquivalentBasePrice=&quot;vasto rex aeolus&quot; Taxes=&quot;ast ego&quot; Fees=&quot;omnipotens speluncis abdidit&quot; Services=&quot;rex aeolus&quot; ApproximateTaxes=&quot;temperat iras&quot; ApproximateFees=&quot;nimborum in patriam&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;rail:RailFare Key=&quot;e nubibus&quot; FareBasis=&quot;iunonis adorat praeterea&quot; CabinClass=&quot;claustra fremunt&quot; PassengerTypeCode=&quot;gente&quot; Origin=&quot;tra&quot; Destination=&quot;nim&quot; EffectiveDate=&quot;divum incedo&quot; Amount=&quot;illum exspirantem&quot; RouteDescription=&quot;venit hic vasto&quot; TicketTypeCode=&quot;transfixo pectore flammas&quot; FareReference=&quot;et soror&quot; CrossCityFare=&quot;false&quot; OriginStationName=&quot;celsa sedet&quot; DestinationStationName=&quot;speluncis abdidit&quot; ReservationRequired=&quot;true&quot; JourneyDirection=&quot;Return&quot; RailLocOrigin=&quot;supplex &quot; RailLocDestination=&quot;infixit &quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;rail:RailFareNoteRef Key=&quot;in patriam loca&quot;/>
                  &lt;rail:RailFareID Key=&quot;pater omnipotens&quot; Category=&quot;insuper altos&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>aut supplex&lt;/rail:RailFareID>
                  &lt;rail:RailFareIDRef Key=&quot;gente tot annos&quot;/>
                  &lt;rail:FareValidity RailJourneyRef=&quot;evertitque aequora&quot; NotValidBefore=&quot;2007-07-22-06:00&quot; NotValidAfter=&quot;2006-03-20-07:00&quot;/>
                  &lt;com:HostToken Host=&quot;et qu&quot; Key=&quot;anySimpleType&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>vasto rex aeolus&lt;/com:HostToken>
                  &lt;rail:FulFillmentType>insuper altos&lt;/rail:FulFillmentType>
                  &lt;rail:RailFareComponent Key=&quot;certo et premere&quot; Amount=&quot;iussus habenas&quot; Age=&quot;100&quot; PassengerTypeCode=&quot;cum g&quot; SupplierPassengerType=&quot;rapidum iaculata e&quot; Quantity=&quot;100&quot;>
                     &lt;rail:Discount Key=&quot;tempestatesque sonoras&quot;>
                        &lt;com:DiscountCard Key=&quot;et carcere&quot; Code=&quot;patriam &quot; Description=&quot;gero et quisquam&quot; Number=&quot;claustra fremunt celsa&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;/>
                     &lt;/rail:Discount>
                  &lt;/rail:RailFareComponent>
               &lt;/rail:RailFare>
               &lt;rail:RailFareRef Key=&quot;tempestatesque sonoras imperio&quot;/>
               &lt;rail:RailBookingInfo RailFareRef=&quot;quae divum&quot; RailJourneyRef=&quot;murmure montis&quot; OptionalService=&quot;false&quot;/>
               &lt;rail:PassengerType Code=&quot;aeolu&quot; Age=&quot;100&quot; DOB=&quot;2003-08-09&quot; Gender=&quot;im&quot; PricePTCOnly=&quot;true&quot; BookingTravelerRef=&quot;metuens molemque&quot; AccompaniedPassenger=&quot;false&quot; ResidencyType=&quot;Employee&quot;>
                  &lt;com:Name Prefix=&quot;e nubibus&quot; First=&quot;divum incedo regina&quot; Middle=&quot;venit hic&quot; Last=&quot;sciret dare&quot; Suffix=&quot;pater omnipotens speluncis&quot; TravelerProfileId=&quot;100&quot;/>
                  &lt;com:LoyaltyCard Key=&quot;ac terras&quot; SupplierCode=&quot;et&quot; AllianceLevel=&quot;iovisque et&quot; MembershipProgram=&quot;quae divum&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; CardNumber=&quot;premit ac&quot; Status=&quot;acuto ast ego&quot; MembershipStatus=&quot;volutans nimborum in&quot; FreeText=&quot;circum claustra&quot; SupplierType=&quot;Rail&quot; Level=&quot;profundum quippe&quot; PriorityCode=&quot;rex aeolus antro&quot; VendorLocationRef=&quot;premere et&quot;>
                     &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;true&quot; ReservationLevel=&quot;false&quot;>
                        &lt;com:OperatedBy>mollitque animos et&lt;/com:OperatedBy>
                        &lt;com:ProviderReservationInfoRef Key=&quot;feta furentibus austris&quot;/>
                     &lt;/com:ProviderReservationSpecificInfo>
                  &lt;/com:LoyaltyCard>
                  &lt;com:DiscountCard Key=&quot;quae divum&quot; Code=&quot;ast ego&quot; Description=&quot;secum verrantque&quot; Number=&quot;et laxas sciret&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;/>
                  &lt;com:PersonalGeography>
                     &lt;com:CountryCode>et&lt;/com:CountryCode>
                     &lt;com:StateProvinceCode>ego qu&lt;/com:StateProvinceCode>
                     &lt;com:CityCode>ver&lt;/com:CityCode>
                  &lt;/com:PersonalGeography>
               &lt;/rail:PassengerType>
               &lt;com:BookingTravelerRef Key=&quot;quisquam numen iunonis&quot;>
                  &lt;com:LoyaltyCardRef Key=&quot;annos bella&quot;/>
                  &lt;com:DriversLicenseRef Key=&quot;montis circum&quot;/>
                  &lt;com:DiscountCardRef Key=&quot;scopuloque infixit&quot;/>
                  &lt;com:PaymentRef Key=&quot;foedere certo&quot;/>
               &lt;/com:BookingTravelerRef>
            &lt;/rail:RailPricingInfo>
         &lt;/rail:RailPricingSolution>
         &lt;com:Payment Key=&quot;insuper altos imposuit&quot; Type=&quot;ServiceFee&quot; FormOfPaymentRef=&quot;aeolus antro&quot; BookingTravelerRef=&quot;regina iovisque et&quot; Amount=&quot;secum dea&quot; AmountType=&quot;regemque dedit&quot; ApproximateAmount=&quot;ac vinclis&quot; Status=&quot;laxas sciret&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;/>
         &lt;rail:RailFareNoteList>
            &lt;rail:RailFareNote Key=&quot;gente tot&quot; NoteName=&quot;et quisquam numen&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>magno cum murmure&lt;/rail:RailFareNote>
         &lt;/rail:RailFareNoteList>
         &lt;com:HostTokenList>
            &lt;com:HostToken Host=&quot;aeolu&quot; Key=&quot;anySimpleType&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>sedet aeolus arce&lt;/com:HostToken>
         &lt;/com:HostTokenList>
         &lt;rail:RailAutoSeatAssignment SeatType=&quot;nimborum in patriam&quot; SeatValue=&quot;honorem talia flammato&quot; RailSegmentRef=&quot;premit ac vinclis&quot; BookingTravelerRef=&quot;evertitque aequora&quot;/>
         &lt;rail:RailSpecificSeatAssignment CoachLabel=&quot;ni faciat maria&quot; PlaceLabel=&quot;qui foedere certo&quot; Assignment=&quot;sciret dare&quot; RailSegmentRef=&quot;et quisquam&quot; BookingTravelerRef=&quot;aeolus arce sceptra&quot;/>
      &lt;/univ:RailCreateReservationReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/RailService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
