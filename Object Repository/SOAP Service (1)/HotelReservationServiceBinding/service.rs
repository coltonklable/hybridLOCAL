<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>41333b6f-bf29-4647-84a7-8b6f3cbd66bf</elementGuidId>
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
      <value>http://localhost:8080/kestrel/HotelService</value>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot; xmlns:hot=&quot;http://www.travelport.com/schema/hotel_v51_0&quot;>
   &lt;soapenv:Header>
      &lt;univ:SupportedVersions urVersion=&quot;regemque dedit&quot; airVersion=&quot;adorat praeterea&quot; hotelVersion=&quot;cum gente&quot; vehicleVersion=&quot;rates evertitque&quot; passiveVersion=&quot;ventis illum exspirantem&quot; railVersion=&quot;rex aeolus&quot; cruiseVersion=&quot;atris hoc&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:HotelCreateReservationReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; RuleName=&quot;regemque d&quot; UniversalRecordLocatorCode=&quot;fremunt &quot; ProviderLocatorCode=&quot;ferant r&quot; ProviderCode=&quot;sceptra tenens&quot; CustomerNumber=&quot;turbine corripuit scopuloque&quot; Version=&quot;100&quot; UserAcceptance=&quot;false&quot; MandatoryRateMatch=&quot;false&quot; StatusCode=&quot;fl&quot; BookingConfirmation=&quot;profundum quippe ferant&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;nubibus ignem disiecitque&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;ac vi&quot; ProviderCode=&quot;rapid&quot; AgentID=&quot;caelumque profundum&quot;/>
         &lt;com:TerminalSessionInfo>circum claustra&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;coniu&quot; PseudoCityCode=&quot;arce scept&quot;/>
         &lt;com:LinkedUniversalRecord LocatorCode=&quot;circum c&quot; Key=&quot;certo et premere&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;/>
         &lt;com:BookingTraveler Key=&quot;feta furentibus&quot; TravelerType=&quot;sed p&quot; Age=&quot;100&quot; VIP=&quot;false&quot; DOB=&quot;2011-12-24&quot; Gender=&quot;an&quot; Nationality=&quot;ce&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; NameNumber=&quot;volutans nimborum in&quot;>
            &lt;com:BookingTravelerName Prefix=&quot;faciat maria&quot; First=&quot;et quisquam&quot; Middle=&quot;et soror&quot; Last=&quot;annos bella gero&quot; Suffix=&quot;ventos tempestatesque sonoras&quot;/>
            &lt;com:DeliveryInfo Type=&quot;regina iovisque&quot; SignatureRequired=&quot;rapidum ia&quot; TrackingNumber=&quot;insuper altos&quot;>
               &lt;com:ShippingAddress Key=&quot;cum murmure&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>abdidit atris hoc&lt;/com:AddressName>
                  &lt;com:Street>coniunx una cum&lt;/com:Street>
                  &lt;com:City>ipsa iovis&lt;/com:City>
                  &lt;com:State>frenat illi indignantes&lt;/com:State>
                  &lt;com:PostalCode>nimborum in&lt;/com:PostalCode>
                  &lt;com:Country>mo&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;montis insuper altos&quot;/>
               &lt;/com:ShippingAddress>
               &lt;com:PhoneNumber Key=&quot;molemque et montis&quot; Type=&quot;Business&quot; Location=&quot;nimborum i&quot; CountryCode=&quot;vasto&quot; AreaCode=&quot;flammato s&quot; Number=&quot;tempestatesque sonoras&quot; Extension=&quot;tenens mol&quot; Text=&quot;premit ac&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;nubibus ignem&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:Email Key=&quot;austris aeoliam venit&quot; Type=&quot;qui foedere&quot; Comment=&quot;atris hoc metuens&quot; EmailID=&quot;atris hoc&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;sonoras imperio&quot;/>
               &lt;/com:Email>
               &lt;com:GeneralRemark Key=&quot;patriam loca&quot; Category=&quot;montis circum&quot; TypeInGds=&quot;laxas sciret dare&quot; SupplierType=&quot;Rail&quot; ProviderReservationInfoRef=&quot;sceptra tenens mollitque&quot; ProviderCode=&quot;vasto&quot; SupplierCode=&quot;coniu&quot; Direction=&quot;Outgoing&quot; CreateDate=&quot;2008-11-03T13:17:16&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:RemarkData>aequora ventis illum&lt;/com:RemarkData>
                  &lt;com:BookingTravelerRef>aequora ventis&lt;/com:BookingTravelerRef>
               &lt;/com:GeneralRemark>
               &lt;com:ProviderReservationInfoRef Key=&quot;et coniunx&quot;/>
            &lt;/com:DeliveryInfo>
            &lt;com:PhoneNumber Key=&quot;incedo regina iovisque&quot; Type=&quot;Home&quot; Location=&quot;arce scept&quot; CountryCode=&quot;ni fa&quot; AreaCode=&quot;circum cla&quot; Number=&quot;corde volutans&quot; Extension=&quot;insuper al&quot; Text=&quot;temperat iras&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;gero et&quot;/>
            &lt;/com:PhoneNumber>
            &lt;com:Email Key=&quot;iunonis adorat praeterea&quot; Type=&quot;premere et&quot; Comment=&quot;iovisque et&quot; EmailID=&quot;nubibus ignem disiecitque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;rates evertitque&quot;/>
            &lt;/com:Email>
            &lt;com:LoyaltyCard Key=&quot;supplex aris imponet&quot; SupplierCode=&quot;ip&quot; AllianceLevel=&quot;volutans nimborum in&quot; MembershipProgram=&quot;maria ac&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot; CardNumber=&quot;sciret dare iussus&quot; Status=&quot;rapidi secum verrantque&quot; MembershipStatus=&quot;montis circum&quot; FreeText=&quot;et laxas sciret&quot; SupplierType=&quot;Cruise&quot; Level=&quot;talia flammato&quot; PriorityCode=&quot;terras caelumque&quot; VendorLocationRef=&quot;et carcere&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;false&quot; ReservationLevel=&quot;true&quot;>
                  &lt;com:OperatedBy>vasto rex aeolus&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;arce sceptra&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;luctantis ventos&quot; Code=&quot;bella ge&quot; Description=&quot;ac vinclis et&quot; Number=&quot;imposuit regemque&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
            &lt;com:SSR Key=&quot;et premere&quot; SegmentRef=&quot;murmure montis&quot; PassiveSegmentRef=&quot;premere et&quot; ProviderReservationInfoRef=&quot;rapidi secum&quot; Type=&quot;rapi&quot; Status=&quot;premere et&quot; FreeText=&quot;insuper altos&quot; Carrier=&quot;ma&quot; CarrierSpecificText=&quot;rates evertitque&quot; Description=&quot;iovis rapidum&quot; ProviderDefinedType=&quot;indignantes magn&quot; SSRRuleRef=&quot;sonoras imperio&quot; URL=&quot;http://www.company.edu/secum/auras&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot; ProfileID=&quot;flammato secum dea&quot; ProfileSecureFlightDocKey=&quot;hic vasto rex&quot;/>
            &lt;com:NameRemark Key=&quot;aeoliam venit&quot; Category=&quot;altos imposuit regemque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:RemarkData>premit ac vinclis&lt;/com:RemarkData>
               &lt;com:ProviderReservationInfoRef Key=&quot;luctantis ventos&quot;/>
            &lt;/com:NameRemark>
            &lt;com:AirSeatAssignment Key=&quot;luctantis ventos tempestatesque&quot; Status=&quot;nu&quot; Seat=&quot;et soror&quot; SeatTypeCode=&quot;anno&quot; SegmentRef=&quot;ignem disiecitque&quot; FlightDetailsRef=&quot;iovisque et soror&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; RailCoachNumber=&quot;montis insuper&quot;/>
            &lt;com:RailSeatAssignment Key=&quot;profundum quippe&quot; Status=&quot;be&quot; Seat=&quot;incedo regina iovisque&quot; RailSegmentRef=&quot;iovis rapidum&quot; CoachNumber=&quot;et premere et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
               &lt;com:Characteristic SeatType=&quot;transfixo pectore flammas&quot; SeatDescription=&quot;claustra fremunt&quot; SeatValue=&quot;una cum&quot; SeatValueDescription=&quot;corde volutans nimborum&quot;/>
            &lt;/com:RailSeatAssignment>
            &lt;com:EmergencyInfo>et soror et&lt;/com:EmergencyInfo>
            &lt;com:Address Key=&quot;quae divum&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
               &lt;com:AddressName>sceptra tenens mollitque&lt;/com:AddressName>
               &lt;com:Street>volutans nimborum in&lt;/com:Street>
               &lt;com:City>venit hic&lt;/com:City>
               &lt;com:State>et laxas&lt;/com:State>
               &lt;com:PostalCode>ferant rapidi&lt;/com:PostalCode>
               &lt;com:Country>ar&lt;/com:Country>
               &lt;com:ProviderReservationInfoRef Key=&quot;ignem disiecitque rates&quot;/>
            &lt;/com:Address>
            &lt;com:DriversLicense Key=&quot;pater omnipotens&quot; LicenseNumber=&quot;sedet aeolus&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:AppliedProfile Key=&quot;mollitque animos&quot; TravelerID=&quot;bella gero&quot; TravelerName=&quot;ast ego&quot; AccountID=&quot;nubibus ignem&quot; AccountName=&quot;aut supplex&quot; ImmediateParentID=&quot;ni faciat&quot; ImmediateParentName=&quot;dare iussus habenas&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:CustomizedNameData Key=&quot;claustra fremunt celsa&quot; ProviderReservationInfoRef=&quot;pater omnipotens&quot;>quippe ferant rapidi&lt;/com:CustomizedNameData>
            &lt;com:TravelComplianceData Key=&quot;regina iovisque&quot; AirSegmentRef=&quot;exspirantem transfixo&quot; PassiveSegmentRef=&quot;honorem talia flammato&quot; RailSegmentRef=&quot;mollitque animos&quot; ReservationLocatorRef=&quot;rates ev&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
               &lt;com:PolicyCompliance InPolicy=&quot;true&quot; PolicyToken=&quot;sedet aeolus&quot;/>
               &lt;com:ContractCompliance InContract=&quot;false&quot; ContractToken=&quot;illi indignantes&quot;/>
               &lt;com:PreferredSupplier Preferred=&quot;false&quot; ProfileType=&quot;Agency&quot;/>
            &lt;/com:TravelComplianceData>
            &lt;com:TravelInfo TripName=&quot;molemque et montis&quot; TravelPurpose=&quot;mollitque animos et&quot;/>
         &lt;/com:BookingTraveler>
         &lt;com:OSI Key=&quot;tenens mollitque&quot; Carrier=&quot;ci&quot; Code=&quot;circ&quot; Text=&quot;acuto ast&quot; ProviderReservationInfoRef=&quot;honorem talia flammato&quot; ProviderCode=&quot;ac te&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;/>
         &lt;com:AccountingRemark Key=&quot;flammas turbine corripuit&quot; Category=&quot;ipsa iovis&quot; TypeInGds=&quot;sedet aeolus&quot; ProviderReservationInfoRef=&quot;frenat illi&quot; ProviderCode=&quot;vincl&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>talia flammato&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>auras sed&lt;/com:BookingTravelerRef>
         &lt;/com:AccountingRemark>
         &lt;com:GeneralRemark Key=&quot;ventis illum exspirantem&quot; Category=&quot;rapidum iaculata&quot; TypeInGds=&quot;claustra fremunt&quot; SupplierType=&quot;Cruise&quot; ProviderReservationInfoRef=&quot;caelumque profundum quippe&quot; ProviderCode=&quot;tenen&quot; SupplierCode=&quot;rates&quot; Direction=&quot;Incoming&quot; CreateDate=&quot;2003-08-03T08:28:31&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>patriam loca feta&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>molemque et&lt;/com:BookingTravelerRef>
         &lt;/com:GeneralRemark>
         &lt;com:XMLRemark Key=&quot;animos et&quot; Category=&quot;animos et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>ventos tempestatesque&lt;/com:XMLRemark>
         &lt;com:UnassociatedRemark ProviderReservationInfoRef=&quot;et temperat&quot; ProviderCode=&quot;foede&quot; Key=&quot;verrantque per auras&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>per auras sed&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>vinclis et&lt;/com:BookingTravelerRef>
         &lt;/com:UnassociatedRemark>
         &lt;com:Postscript ProviderReservationInfoRef=&quot;montis insuper altos&quot; ProviderCode=&quot;acuto&quot; Key=&quot;secum dea&quot;>iras ni&lt;/com:Postscript>
         &lt;com:PassiveInfo ProviderCode=&quot;et quisquam&quot; ProviderLocatorCode=&quot;carcere frenat illi&quot; SupplierCode=&quot;iras ni&quot; SupplierLocatorCode=&quot;iras ni&quot;>
            &lt;com:TicketNumber>tempestatesque sonoras imperio&lt;/com:TicketNumber>
            &lt;com:ConfirmationNumber>austris aeoliam&lt;/com:ConfirmationNumber>
            &lt;com:Commission Key=&quot;divum incedo&quot; Level=&quot;Fare&quot; Type=&quot;PercentBase&quot; Modifier=&quot;SupplementaryAmount&quot; Amount=&quot;aeoliam venit hic&quot; Value=&quot;talia flammato&quot; Percentage=&quot;caelumque profundum&quot; BookingTravelerRef=&quot;aeoliam venit&quot; CommissionOverride=&quot;false&quot;/>
         &lt;/com:PassiveInfo>
         &lt;com:ContinuityCheckOverride Key=&quot;nubibus ignem&quot;>rapidi secum&lt;/com:ContinuityCheckOverride>
         &lt;com:AgencyContactInfo Key=&quot;et carcere frenat&quot;>
            &lt;com:PhoneNumber Key=&quot;austris aeoliam&quot; Type=&quot;Agency&quot; Location=&quot;patriam lo&quot; CountryCode=&quot;insup&quot; AreaCode=&quot;et carcere&quot; Number=&quot;laxas sciret&quot; Extension=&quot;infixit ac&quot; Text=&quot;abdidit atris&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;annos bella&quot;/>
            &lt;/com:PhoneNumber>
         &lt;/com:AgencyContactInfo>
         &lt;com:CustomerID ProviderReservationInfoRef=&quot;soror et&quot; ProviderCode=&quot;annos&quot; Key=&quot;qui foedere&quot;>gero et&lt;/com:CustomerID>
         &lt;com:FileFinishingInfo>
            &lt;com:ShopInformation CabinShopped=&quot;speluncis abdidit atris&quot; CabinSelected=&quot;per auras&quot; LowestFareOffered=&quot;altos imposuit regemque&quot;>
               &lt;com:SearchRequest Origin=&quot;aeo&quot; Destination=&quot;ven&quot; DepartureTime=&quot;altos imposuit&quot; ClassOfService=&quot;ve&quot;/>
               &lt;com:FlightsOffered Origin=&quot;ius&quot; Destination=&quot;aeo&quot; DepartureTime=&quot;ignem disiecitque rates&quot; TravelOrder=&quot;3&quot; Carrier=&quot;ge&quot; FlightNumber=&quot;impon&quot; ClassOfService=&quot;sc&quot; StopOver=&quot;false&quot; Connection=&quot;false&quot;/>
            &lt;/com:ShopInformation>
            &lt;com:PolicyInformation Type=&quot;Rail&quot; Name=&quot;anySimpleType&quot; OutOfPolicy=&quot;false&quot; SegmentRef=&quot;scopuloque infixit&quot;>
               &lt;com:ReasonCode>
                  &lt;com:OutOfPolicy>ni faciat maria&lt;/com:OutOfPolicy>
                  &lt;com:PurposeOfTrip>regemque dedit qui&lt;/com:PurposeOfTrip>
                  &lt;com:Remark Key=&quot;speluncis abdidit&quot;>aeolus arce sceptra&lt;/com:Remark>
               &lt;/com:ReasonCode>
            &lt;/com:PolicyInformation>
            &lt;com:AccountInformation AccountName=&quot;imperio premit ac&quot;>
               &lt;com:Address Key=&quot;ni faciat&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>faciat maria&lt;/com:AddressName>
                  &lt;com:Street>ignem disiecitque&lt;/com:Street>
                  &lt;com:City>illum exspirantem&lt;/com:City>
                  &lt;com:State>et premere&lt;/com:State>
                  &lt;com:PostalCode>volutans nimbor&lt;/com:PostalCode>
                  &lt;com:Country>fe&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;et temperat&quot;/>
               &lt;/com:Address>
               &lt;com:PhoneNumber Key=&quot;habenas ipsa&quot; Type=&quot;Email&quot; Location=&quot;laxas scir&quot; CountryCode=&quot;maria&quot; AreaCode=&quot;furentibus&quot; Number=&quot;corde volutans&quot; Extension=&quot;disiecitqu&quot; Text=&quot;molemque et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;atris hoc&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AccountInformation>
            &lt;com:AgencyInformation>
               &lt;com:Address Key=&quot;flammato secum&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>fremunt celsa&lt;/com:AddressName>
                  &lt;com:Street>aut supplex&lt;/com:Street>
                  &lt;com:City>et montis insuper&lt;/com:City>
                  &lt;com:State>sedet aeolus&lt;/com:State>
                  &lt;com:PostalCode>iras ni&lt;/com:PostalCode>
                  &lt;com:Country>co&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;quae divum&quot;/>
               &lt;/com:Address>
               &lt;com:Email Key=&quot;circum claustra fremunt&quot; Type=&quot;et montis&quot; Comment=&quot;speluncis abdidit&quot; EmailID=&quot;rapidi secum&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;dea corde volutans&quot;/>
               &lt;/com:Email>
               &lt;com:PhoneNumber Key=&quot;ferant rapidi&quot; Type=&quot;Hotel&quot; Location=&quot;celsa sede&quot; CountryCode=&quot;exspi&quot; AreaCode=&quot;coniunx un&quot; Number=&quot;aequora ventis illum&quot; Extension=&quot;disiecitqu&quot; Text=&quot;supplex aris&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;abdidit atris&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AgencyInformation>
            &lt;com:TravelerInformation HomeAirport=&quot;dar&quot; VisaExpirationDate=&quot;2008-03-07-07:00&quot; BookingTravelerRef=&quot;certo et premere&quot;>
               &lt;com:EmergencyContact Name=&quot;anySimpleType&quot; Relationship=&quot;anySimpleType&quot;>
                  &lt;com:PhoneNumber Key=&quot;e nubibus ignem&quot; Type=&quot;Business&quot; Location=&quot;imperio pr&quot; CountryCode=&quot;secum&quot; AreaCode=&quot;regemque d&quot; Number=&quot;corde volutans&quot; Extension=&quot;vinclis et&quot; Text=&quot;circum claustra fremunt&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;pectore flammas&quot;/>
                  &lt;/com:PhoneNumber>
               &lt;/com:EmergencyContact>
            &lt;/com:TravelerInformation>
            &lt;com:CustomProfileInformation/>
         &lt;/com:FileFinishingInfo>
         &lt;com:CommissionRemark Key=&quot;quae divum&quot; ProviderReservationInfoRef=&quot;secum verrantque&quot; ProviderCode=&quot;circu&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:ProviderReservationLevel Amount=&quot;quae divum&quot; Percentage=&quot;ni faciat&quot; CommissionCap=&quot;caelumque profundum quippe&quot;/>
            &lt;com:PassengerTypeLevel TravelerType=&quot;aeolu&quot; Amount=&quot;illi indignantes&quot; Percentage=&quot;ac vinclis&quot; CommissionCap=&quot;habenas ipsa&quot;/>
         &lt;/com:CommissionRemark>
         &lt;com:ConsolidatorRemark Key=&quot;sed pater omnipotens&quot; ProviderReservationInfoRef=&quot;iovis rapidum iaculata&quot; ProviderCode=&quot;ni fa&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:PseudoCityCode>ipsa iovis&lt;/com:PseudoCityCode>
         &lt;/com:ConsolidatorRemark>
         &lt;com:InvoiceRemark ProviderReservationInfoRef=&quot;dedit qui foedere&quot; ProviderCode=&quot;ipsa &quot; Key=&quot;patriam loca&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>tenens mollitque&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>exspirantem transfixo&lt;/com:BookingTravelerRef>
            &lt;com:AirSegmentRef Key=&quot;exspirantem transfixo&quot;/>
            &lt;com:HotelReservationRef LocatorCode=&quot;vinclis &quot;/>
            &lt;com:VehicleReservationRef LocatorCode=&quot;divum in&quot;/>
            &lt;com:PassiveSegmentRef Key=&quot;furentibus austris&quot;/>
         &lt;/com:InvoiceRemark>
         &lt;com:SSR Key=&quot;volutans nimborum&quot; SegmentRef=&quot;claustra fremunt celsa&quot; PassiveSegmentRef=&quot;et laxas sciret&quot; ProviderReservationInfoRef=&quot;habenas ipsa&quot; Type=&quot;gent&quot; Status=&quot;premere et&quot; FreeText=&quot;et montis&quot; Carrier=&quot;eg&quot; CarrierSpecificText=&quot;dea corde volutans&quot; Description=&quot;celsa sedet&quot; ProviderDefinedType=&quot;flammato secum&quot; SSRRuleRef=&quot;et laxas&quot; URL=&quot;http://www.any.org/nimborum/vasto&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; ProfileID=&quot;e nubibus&quot; ProfileSecureFlightDocKey=&quot;aris imponet&quot;/>
         &lt;com:EmailNotification Recipients=&quot;Specific&quot;>
            &lt;com:EmailRef>corde volutans&lt;/com:EmailRef>
         &lt;/com:EmailNotification>
         &lt;com:QueuePlace>
            &lt;com:PseudoCityCode>iaculata e&lt;/com:PseudoCityCode>
            &lt;com:QueueSelector Queue=&quot;carcere frenat illi&quot; Category=&quot;aut supplex aris&quot; DateRange=&quot;molemque et&quot;/>
         &lt;/com:QueuePlace>
         &lt;com:FormOfPayment Key=&quot;feta furentibus&quot; Type=&quot;bella gero&quot; FulfillmentType=&quot;ac terras&quot; FulfillmentLocation=&quot;et montis&quot; FulfillmentIDType=&quot;Euro Cheque Card&quot; FulfillmentIDNumber=&quot;quippe ferant&quot; IsAgentType=&quot;false&quot; AgentText=&quot;austris aeoliam&quot; ReuseFOP=&quot;infixit acuto&quot; ExternalReference=&quot;animos et&quot; Reusable=&quot;false&quot; ProfileID=&quot;aequora ventis illum&quot; ProfileKey=&quot;et temperat&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
            &lt;com:CreditCard Type=&quot;su&quot; Number=&quot;iovis rapidum&quot; ExpDate=&quot;2018-02-07:00&quot; Name=&quot;una cum gente&quot; CVV=&quot;sono&quot; ApprovalCode=&quot;patriam loca fet&quot; ExtendedPayment=&quot;gero et&quot; CustomerReference=&quot;ac terras caelumque&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;aequora ventis&quot; BankCountryCode=&quot;tr&quot; BankStateCode=&quot;iovis &quot; Enett=&quot;false&quot; ProfileID=&quot;venit hic&quot; Key=&quot;flammas turbine corripuit&quot;>
               &lt;com:PhoneNumber Key=&quot;foedere certo et&quot; Type=&quot;Hotel&quot; Location=&quot;abdidit at&quot; CountryCode=&quot;evert&quot; AreaCode=&quot;illi indig&quot; Number=&quot;terras caelumque&quot; Extension=&quot;rates ever&quot; Text=&quot;ni faciat maria&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;aeolus antro luctantis&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;iussus habenas ipsa&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>antro luctantis&lt;/com:AddressName>
                  &lt;com:Street>terras caelumque&lt;/com:Street>
                  &lt;com:City>vasto rex&lt;/com:City>
                  &lt;com:State>honorem talia flammato&lt;/com:State>
                  &lt;com:PostalCode>circum claustra&lt;/com:PostalCode>
                  &lt;com:Country>ve&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;nubibus ignem&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:DebitCard Type=&quot;ce&quot; Number=&quot;vinclis etvinclis et&quot; ExpDate=&quot;2013-12&quot; Name=&quot;acuto ast&quot; CVV=&quot;et l&quot; ApprovalCode=&quot;montis insuper a&quot; IssueNumber=&quot;quae div&quot; ProfileID=&quot;talia flammato&quot; Key=&quot;aeoliam venit&quot;>
               &lt;com:PhoneNumber Key=&quot;rapidi secum&quot; Type=&quot;Other&quot; Location=&quot;et coniunx&quot; CountryCode=&quot;insup&quot; AreaCode=&quot;annos bell&quot; Number=&quot;rapidum iaculata e&quot; Extension=&quot;quisquam n&quot; Text=&quot;montis insuper altos&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;iovisque et&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;tot annos bella&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>et montis insuper&lt;/com:AddressName>
                  &lt;com:Street>magno cum&lt;/com:Street>
                  &lt;com:City>infixit acuto ast&lt;/com:City>
                  &lt;com:State>tot annos&lt;/com:State>
                  &lt;com:PostalCode>ni faciat&lt;/com:PostalCode>
                  &lt;com:Country>ae&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;volutans nimborum&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:DebitCard>
            &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
            &lt;com:Certificate Number=&quot;hoc metuens&quot; Amount=&quot;luctantis ventos tempestatesque&quot; DiscountAmount=&quot;ast ego&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2011-04-17-06:00&quot; NotValidAfter=&quot;2017-01-26&quot;/>
            &lt;com:TicketNumber>exspirantem t&lt;/com:TicketNumber>
            &lt;com:Check MICRNumber=&quot;sceptra tenens&quot; RoutingNumber=&quot;aequora ventis&quot; AccountNumber=&quot;regina iovisque&quot; CheckNumber=&quot;sed pater&quot;/>
            &lt;com:Requisition Number=&quot;cum gente&quot; Category=&quot;Other&quot; Type=&quot;Cash&quot;/>
            &lt;com:MiscFormOfPayment CreditCardType=&quot;ca&quot; CreditCardNumber=&quot;una cumuna cum&quot; ExpDate=&quot;2002-01&quot; Text=&quot;montis insuper&quot; Category=&quot;ego quae divum&quot; AcceptanceOverride=&quot;false&quot;/>
            &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;ego quae&quot; AgencyBillingNumber=&quot;imponet honorem&quot; AgencyBillingPassword=&quot;nimborum in&quot;/>
            &lt;com:UnitedNations Number=&quot;rapidum iaculata&quot;/>
            &lt;com:DirectPayment Text=&quot;ventis illum exspirantem&quot;/>
            &lt;com:AgentVoucher Number=&quot;sedet aeolus&quot;/>
            &lt;com:PaymentAdvice Type=&quot;mol&quot; DocumentNumber=&quot;infixit acuto&quot; IssueDate=&quot;2017-06-17&quot; IssueCity=&quot;mag&quot; OriginalFOP=&quot;supplex aris impone&quot;/>
            &lt;com:ProviderReservationInfoRef Key=&quot;animos et&quot; ProviderReservationLevel=&quot;true&quot;/>
            &lt;com:SegmentRef Key=&quot;illum exspirantem&quot;/>
            &lt;com:BSPPayment BSPIdentifier=&quot;cum murmure&quot; BSPPassword=&quot;dare iussus&quot;/>
            &lt;com:ARCPayment ARCIdentifier=&quot;rex aeolus antro&quot; ARCPassword=&quot;circum claustra&quot;/>
         &lt;/com:FormOfPayment>
         &lt;com:Email Key=&quot;sceptra tenens&quot; Type=&quot;tenens mollitque&quot; Comment=&quot;altos imposuit regemque&quot; EmailID=&quot;aeolus arce&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:ProviderReservationInfoRef Key=&quot;verrantque per&quot;/>
         &lt;/com:Email>
         &lt;com:PhoneNumber Key=&quot;rapidi secum verrantque&quot; Type=&quot;Agency&quot; Location=&quot;illum exsp&quot; CountryCode=&quot;rates&quot; AreaCode=&quot;bella gero&quot; Number=&quot;et montis&quot; Extension=&quot;quippe fer&quot; Text=&quot;auras sed pater&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:ProviderReservationInfoRef Key=&quot;animos et temperat&quot;/>
         &lt;/com:PhoneNumber>
         &lt;hot:HotelRateDetail RatePlanType=&quot;et premere&quot; Base=&quot;dare iussus habenas&quot; Tax=&quot;hic vasto&quot; Total=&quot;ventis illum&quot; Surcharge=&quot;et montis&quot; ApproximateBase=&quot;una cum gente&quot; ApproximateTax=&quot;qui foedere&quot; ApproximateTotal=&quot;aeolus antro&quot; ApproximateSurcharge=&quot;rapidum iaculata&quot; RateGuaranteed=&quot;false&quot; ApproximateRateGuaranteed=&quot;false&quot; RateCategory=&quot;201&quot; Key=&quot;temperat iras&quot; RateSupplier=&quot;gero et&quot; BookableQuantity=&quot;201&quot; RateOfferId=&quot;illi indignantes&quot; InPolicy=&quot;false&quot; RateChangeIndicator=&quot;false&quot; ExtraFeesIncluded=&quot;unknown&quot;>
            &lt;hot:PolicyCodesList>
               &lt;com:PolicyCode>tot annos&lt;/com:PolicyCode>
               &lt;com:MinPolicyCode>flammas turbine&lt;/com:MinPolicyCode>
               &lt;com:MaxPolicyCode>e nubibus ignem&lt;/com:MaxPolicyCode>
            &lt;/hot:PolicyCodesList>
            &lt;hot:RoomRateDescription Name=&quot;ignem disiecitque&quot;>
               &lt;hot:Text>quae divum&lt;/hot:Text>
            &lt;/hot:RoomRateDescription>
            &lt;hot:HotelRateByDate EffectiveDate=&quot;altos imposuit&quot; ExpireDate=&quot;secum verrantque per&quot; Base=&quot;mollitque animos&quot; Tax=&quot;regemque dedit&quot; Total=&quot;metuens molemque&quot; Surcharge=&quot;imposuit regemque&quot; ApproximateBase=&quot;iras ni&quot; ApproximateTotal=&quot;molemque et&quot; Contents=&quot;iussus habenas ipsa&quot;/>
            &lt;com:CorporateDiscountID NegotiatedRateCode=&quot;true&quot;>rex aeolus&lt;/com:CorporateDiscountID>
            &lt;hot:AcceptedPayment PaymentCode=&quot;se&quot;/>
            &lt;hot:Commission Indicator=&quot;true&quot; Percent=&quot;rapidum iaculata e&quot; CommissionAmount=&quot;exspirantem transfixo&quot; ApproxCommissionAmount=&quot;venit hic vasto&quot; CommissionOnSurcharges=&quot;frenat illi&quot; ApproxCommissionOnSurcharges=&quot;cum gente&quot;/>
            &lt;hot:RateMatchIndicator Type=&quot;AdultRollaway&quot; Status=&quot;NotAvailable&quot; Value=&quot;corripuit scopuloque infixit&quot;/>
            &lt;hot:TaxDetails>
               &lt;hot:Tax Code=&quot;201&quot; EffectiveDate=&quot;2001-06-18-06:00&quot; ExpirationDate=&quot;2003-11-27&quot; Term=&quot;frenat illi&quot; CollectionFreq=&quot;celsa sedet&quot;>
                  &lt;hot:Amount>sceptra tenens&lt;/hot:Amount>
                  &lt;hot:Percentage>1.25&lt;/hot:Percentage>
               &lt;/hot:Tax>
            &lt;/hot:TaxDetails>
            &lt;hot:CancelInfo NonRefundableStayIndicator=&quot;unknown&quot; CancelDeadline=&quot;2007-10-14T10:26:14-06:00&quot; TaxInclusive=&quot;false&quot; FeeInclusive=&quot;false&quot; CancelPenaltyAmount=&quot;laxas sciret&quot; NumberOfNights=&quot;200&quot; CancelPenaltyPercent=&quot;1.25&quot; CancelPenaltyPercentAppliesTo=&quot;arce sceptra&quot; Method=&quot;bella gero&quot; Supported=&quot;false&quot; URL=&quot;ventis illum exspirantem&quot; OffsetTimeUnit=&quot;metuens molemque&quot; OffsetUnitMultiplier=&quot;100&quot; OffsetDropTime=&quot;ferant rapidi secum&quot;>
               &lt;hot:CancellationPolicy>tempestatesque sonoras imperio&lt;/hot:CancellationPolicy>
               &lt;hot:Text>exspirantem transfixo&lt;/hot:Text>
            &lt;/hot:CancelInfo>
            &lt;hot:GuaranteeInfo AbsoluteDeadline=&quot;2000-06-10T11:49:30&quot; CredentialsRequired=&quot;true&quot; HoldTime=&quot;corripuit scopuloque infixit&quot; GuaranteeType=&quot;Guarantee&quot; OffsetTimeUnit=&quot;iussus habenas&quot; OffsetUnitMultiplier=&quot;100&quot; OffsetDropTime=&quot;et laxas&quot;>
               &lt;hot:DepositAmount Amount=&quot;acuto ast ego&quot; ApproximateAmount=&quot;murmure montis circum&quot;/>
               &lt;hot:DepositNights>100&lt;/hot:DepositNights>
               &lt;hot:DepositPercent>100&lt;/hot:DepositPercent>
               &lt;hot:GuaranteePaymentType Type=&quot;aequora ventis&quot; Description=&quot;ventos tempestatesque sonoras&quot;>secum verrantque per&lt;/hot:GuaranteePaymentType>
            &lt;/hot:GuaranteeInfo>
            &lt;hot:SupplementalRateInfo>una cum&lt;/hot:SupplementalRateInfo>
            &lt;hot:RoomCapacity IsPackage=&quot;true&quot;>
               &lt;hot:Capacity>201&lt;/hot:Capacity>
            &lt;/hot:RoomCapacity>
            &lt;hot:ExtraCharges ExtraAdultAmount=&quot;ferant rapidi&quot; ExtraChildAmount=&quot;sed pater&quot; CribAmount=&quot;imposuit regemque&quot; AdultRollawayCharge=&quot;iovis rapidum&quot; ChildRollawayCharge=&quot;dea corde&quot;/>
            &lt;hot:Inclusions SmokingRoomIndicator=&quot;unknown&quot;>
               &lt;hot:BedTypes Code=&quot;201&quot; Quantity=&quot;200&quot;/>
               &lt;hot:MealPlans Breakfast=&quot;unknown&quot; Lunch=&quot;true&quot; Dinner=&quot;true&quot;>
                  &lt;hot:MealPlan Code=&quot;201&quot;/>
               &lt;/hot:MealPlans>
               &lt;hot:RoomView Code=&quot;201&quot;/>
            &lt;/hot:Inclusions>
         &lt;/hot:HotelRateDetail>
         &lt;hot:HotelProperty HotelChain=&quot;et&quot; HotelCode=&quot;transfixo pectore&quot; HotelLocation=&quot;sedet &quot; Name=&quot;luctantis ventos tempestatesque&quot; VendorLocationKey=&quot;venit hic vasto&quot; HotelTransportation=&quot;201&quot; ReserveRequirement=&quot;Deposit&quot; ParticipationLevel=&quot;c&quot; Availability=&quot;AvailableForOtherRates&quot; Key=&quot;sedet aeolus arce&quot; PreferredOption=&quot;true&quot; MoreRates=&quot;false&quot; MoreRatesToken=&quot;mollitque animos&quot; NetTransCommissionInd=&quot;P&quot; NumOfRatePlans=&quot;201&quot;>
            &lt;hot:PropertyAddress>
               &lt;hot:Address>illum exspirantem&lt;/hot:Address>
            &lt;/hot:PropertyAddress>
            &lt;com:PhoneNumber Key=&quot;venit hic vasto&quot; Type=&quot;Hotel&quot; Location=&quot;laxas scir&quot; CountryCode=&quot;honor&quot; AreaCode=&quot;ignem disi&quot; Number=&quot;et quisquam numen&quot; Extension=&quot;sceptra te&quot; Text=&quot;dare iussus&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;iovisque et&quot;/>
            &lt;/com:PhoneNumber>
            &lt;com:CoordinateLocation latitude=&quot;1.30&quot; longitude=&quot;1.30&quot;/>
            &lt;com:Distance Units=&quot;MI&quot; Value=&quot;100&quot; Direction=&quot;et&quot;/>
            &lt;hot:HotelRating RatingProvider=&quot;sedet aeolus&quot;>
               &lt;hot:Rating>100&lt;/hot:Rating>
               &lt;hot:RatingRange MinimumRating=&quot;100&quot; MaximumRating=&quot;100&quot;/>
            &lt;/hot:HotelRating>
            &lt;hot:Amenities>
               &lt;hot:Amenity Code=&quot;201&quot; AmenityType=&quot;qu&quot;/>
            &lt;/hot:Amenities>
            &lt;hot:MarketingMessage>
               &lt;hot:Text>annos bella&lt;/hot:Text>
            &lt;/hot:MarketingMessage>
         &lt;/hot:HotelProperty>
         &lt;com:ThirdPartyInformation ThirdPartyCode=&quot;praet&quot; ThirdPartyLocatorCode=&quot;aequora ventis&quot; ThirdPartyName=&quot;ferant rapidi&quot; ProviderReservationInfoRef=&quot;luctantis ventos&quot; Key=&quot;indignantes magno&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:SegmentRef Key=&quot;regemque dedit&quot;/>
         &lt;/com:ThirdPartyInformation>
         &lt;hot:HotelStay Key=&quot;ac terras&quot;>
            &lt;hot:CheckinDate>2012-02-19&lt;/hot:CheckinDate>
            &lt;hot:CheckoutDate>2003-02-09&lt;/hot:CheckoutDate>
         &lt;/hot:HotelStay>
         &lt;com:Guarantee Type=&quot;tenens mollitque animos&quot; Key=&quot;imperio premit&quot; ReuseFOP=&quot;dare iussus&quot; ExternalReference=&quot;et laxas sciret&quot; Reusable=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:CreditCard Type=&quot;sc&quot; Number=&quot;iunonis adorat praeterea&quot; ExpDate=&quot;2006-08&quot; Name=&quot;tempestatesque sonoras imperio&quot; CVV=&quot;quae&quot; ApprovalCode=&quot;murmure montis&quot; ExtendedPayment=&quot;aeolus arce&quot; CustomerReference=&quot;gente tot annos&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;imponet honorem talia&quot; BankCountryCode=&quot;ma&quot; BankStateCode=&quot;et con&quot; Enett=&quot;false&quot; ProfileID=&quot;e nubibus&quot; Key=&quot;divum incedo regina&quot;>
               &lt;com:PhoneNumber Key=&quot;venit hic&quot; Type=&quot;Mobile&quot; Location=&quot;pectore fl&quot; CountryCode=&quot;insup&quot; AreaCode=&quot;corde volu&quot; Number=&quot;numen iunonis&quot; Extension=&quot;illum exsp&quot; Text=&quot;et quisquam&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;nubibus ignem disiecitque&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:BillingAddress Key=&quot;premit ac&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>bella gero et&lt;/com:AddressName>
                  &lt;com:Street>profundum quippe&lt;/com:Street>
                  &lt;com:City>rex aeolus antro&lt;/com:City>
                  &lt;com:State>premere et&lt;/com:State>
                  &lt;com:PostalCode>exspirantem tra&lt;/com:PostalCode>
                  &lt;com:Country>mo&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;feta furentibus austris&quot;/>
               &lt;/com:BillingAddress>
            &lt;/com:CreditCard>
            &lt;com:OtherGuaranteeInfo Type=&quot;Others&quot;>nimborum in&lt;/com:OtherGuaranteeInfo>
         &lt;/com:Guarantee>
         &lt;hot:HotelSpecialRequest>ac vinclis et&lt;/hot:HotelSpecialRequest>
         &lt;com:PointOfSale ProviderCode=&quot;aris &quot; PseudoCityCode=&quot;celsa sede&quot; Key=&quot;rapidum iaculata&quot; IATA=&quot;et coniu&quot;/>
         &lt;hot:PromotionCode Key=&quot;ventos tempestatesque sonoras&quot;>aequora ventis&lt;/hot:PromotionCode>
         &lt;com:BookingSource Code=&quot;austris aeoliam&quot; Type=&quot;CustomerId&quot;/>
         &lt;hot:HotelBedding Type=&quot;montis circum&quot; NumberOfBeds=&quot;3&quot; Amount=&quot;scopuloque infixit&quot; Content=&quot;foedere certo&quot;/>
         &lt;hot:GuestInformation NumberOfRooms=&quot;3&quot;>
            &lt;hot:NumberOfAdults ExtraAdults=&quot;3&quot; Amount=&quot;insuper altos imposuit&quot; Content=&quot;quippe ferant&quot;>praeterea aut&lt;/hot:NumberOfAdults>
            &lt;hot:NumberOfChildren Count=&quot;3&quot; Amount=&quot;incedo regina&quot;>
               &lt;hot:Age>3&lt;/hot:Age>
            &lt;/hot:NumberOfChildren>
            &lt;hot:ExtraChild Count=&quot;3&quot; Content=&quot;ac vinclis et&quot;/>
         &lt;/hot:GuestInformation>
         &lt;hot:AssociatedRemark ProviderReservationInfoRef=&quot;rex aeolus&quot; ProviderCode=&quot;antro&quot; Key=&quot;praeterea aut&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>altos imposuit&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>celsa sedet&lt;/com:BookingTravelerRef>
         &lt;/hot:AssociatedRemark>
         &lt;com:ReservationName>
            &lt;com:BookingTravelerRef Key=&quot;quae divum&quot;>
               &lt;com:LoyaltyCardRef Key=&quot;patriam loca feta&quot;/>
               &lt;com:DriversLicenseRef Key=&quot;sceptra tenens&quot;/>
               &lt;com:DiscountCardRef Key=&quot;premere et&quot;/>
               &lt;com:PaymentRef Key=&quot;scopuloque infixit acuto&quot;/>
            &lt;/com:BookingTravelerRef>
            &lt;com:NameOverride First=&quot;dedit qui&quot; Last=&quot;aris imponet&quot; Age=&quot;100&quot;/>
         &lt;/com:ReservationName>
         &lt;com:ActionStatus Type=&quot;CXL&quot; TicketDate=&quot;evertitque aequora&quot; Key=&quot;ni faciat maria&quot; ProviderReservationInfoRef=&quot;qui foedere certo&quot; QueueCategory=&quot;sciret dare&quot; AirportCode=&quot;et &quot; ProviderCode=&quot;aeolu&quot; SupplierCode=&quot;acuto&quot; PseudoCityCode=&quot;habenas ip&quot; AccountCode=&quot;altos imposuit&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
            &lt;com:Remark Key=&quot;omnipotens speluncis abdidit&quot;>nimborum in&lt;/com:Remark>
         &lt;/com:ActionStatus>
         &lt;com:HostToken Host=&quot;soror&quot; Key=&quot;anySimpleType&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>dare iussus&lt;/com:HostToken>
         &lt;com:ReviewBooking Key=&quot;aeolus antro luctantis&quot; Queue=&quot;99&quot; QueueCategory=&quot;co&quot; DateTime=&quot;2013-05-23T06:53:13-06:00&quot; PseudoCityCode=&quot;nimborum i&quot; ProviderCode=&quot;feran&quot; ProviderReservationInfoRef=&quot;frenat illi indignantes&quot; Remarks=&quot;ego quae&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
         &lt;hot:HotelCommission>soror et coniunx&lt;/hot:HotelCommission>
      &lt;/univ:HotelCreateReservationReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/HotelService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
