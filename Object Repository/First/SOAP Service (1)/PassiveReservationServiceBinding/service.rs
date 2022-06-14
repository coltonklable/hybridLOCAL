<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>671766af-5f0d-4ee5-9fae-81a35e59cdfd</elementGuidId>
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
      <value>http://localhost:8080/kestrel/PassiveService</value>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot; xmlns:pas=&quot;http://www.travelport.com/schema/passive_v51_0&quot;>
   &lt;soapenv:Header>
      &lt;univ:SupportedVersions urVersion=&quot;montis insuper altos&quot; airVersion=&quot;premit ac&quot; hotelVersion=&quot;iovisque et&quot; vehicleVersion=&quot;tot annos bella&quot; passiveVersion=&quot;omnipotens speluncis abdidit&quot; railVersion=&quot;et montis insuper&quot; cruiseVersion=&quot;magno cum&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:PassiveCreateReservationReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; RuleName=&quot;regemque d&quot; UniversalRecordLocatorCode=&quot;fremunt &quot; ProviderLocatorCode=&quot;ferant r&quot; ProviderCode=&quot;sceptra tenens&quot; CustomerNumber=&quot;turbine corripuit scopuloque&quot; Version=&quot;100&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;flammato secum dea&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;profu&quot; ProviderCode=&quot;nubib&quot; AgentID=&quot;ac vinclis et&quot;/>
         &lt;com:TerminalSessionInfo>rapidum iaculata e&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;caelu&quot; PseudoCityCode=&quot;circum cla&quot;/>
         &lt;com:LinkedUniversalRecord LocatorCode=&quot;coniunx &quot; Key=&quot;arce sceptra&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
         &lt;com:BookingTraveler Key=&quot;certo et premere&quot; TravelerType=&quot;et mo&quot; Age=&quot;100&quot; VIP=&quot;false&quot; DOB=&quot;2000-01-07&quot; Gender=&quot;pe&quot; Nationality=&quot;pe&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; NameNumber=&quot;certo et&quot;>
            &lt;com:BookingTravelerName Prefix=&quot;rates evertitque aeq&quot; First=&quot;volutans nimborum in&quot; Middle=&quot;faciat maria&quot; Last=&quot;et quisquam&quot; Suffix=&quot;et soror&quot;/>
            &lt;com:DeliveryInfo Type=&quot;annos bella gero&quot; SignatureRequired=&quot;ventos tem&quot; TrackingNumber=&quot;regina iovisque&quot;>
               &lt;com:ShippingAddress Key=&quot;rapidum iaculata&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>cum murmure&lt;/com:AddressName>
                  &lt;com:Street>in patriam&lt;/com:Street>
                  &lt;com:City>abdidit atris hoc&lt;/com:City>
                  &lt;com:State>coniunx una cum&lt;/com:State>
                  &lt;com:PostalCode>ipsa iovis&lt;/com:PostalCode>
                  &lt;com:Country>fr&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;nimborum in&quot;/>
               &lt;/com:ShippingAddress>
               &lt;com:PhoneNumber Key=&quot;mollitque animos&quot; Type=&quot;Fax&quot; Location=&quot;insuper al&quot; CountryCode=&quot;feran&quot; AreaCode=&quot;nimborum i&quot; Number=&quot;vasto rex&quot; Extension=&quot;flammato s&quot; Text=&quot;tempestatesque sonoras&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;premit ac&quot;/>
               &lt;/com:PhoneNumber>
               &lt;com:Email Key=&quot;iras ni&quot; Type=&quot;nubibus ignem&quot; Comment=&quot;austris aeoliam venit&quot; EmailID=&quot;qui foedere&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;atris hoc&quot;/>
               &lt;/com:Email>
               &lt;com:GeneralRemark Key=&quot;animos et temperat&quot; Category=&quot;sonoras imperio&quot; TypeInGds=&quot;patriam loca&quot; SupplierType=&quot;Vehicle&quot; ProviderReservationInfoRef=&quot;dare iussus habenas&quot; ProviderCode=&quot;tempe&quot; SupplierCode=&quot;scept&quot; Direction=&quot;Outgoing&quot; CreateDate=&quot;2004-06-16T08:14:16+05:30&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:RemarkData>turbine corripuit&lt;/com:RemarkData>
                  &lt;com:BookingTravelerRef>auras sed&lt;/com:BookingTravelerRef>
               &lt;/com:GeneralRemark>
               &lt;com:ProviderReservationInfoRef Key=&quot;aequora ventis illum&quot;/>
            &lt;/com:DeliveryInfo>
            &lt;com:PhoneNumber Key=&quot;aequora ventis&quot; Type=&quot;Mobile&quot; Location=&quot;aris impon&quot; CountryCode=&quot;imper&quot; AreaCode=&quot;arce scept&quot; Number=&quot;ni faciat&quot; Extension=&quot;circum cla&quot; Text=&quot;corde volutans&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;temperat iras&quot;/>
            &lt;/com:PhoneNumber>
            &lt;com:Email Key=&quot;magno cum&quot; Type=&quot;gero et&quot; Comment=&quot;iunonis adorat praeterea&quot; EmailID=&quot;premere et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;nubibus ignem disiecitque&quot;/>
            &lt;/com:Email>
            &lt;com:LoyaltyCard Key=&quot;imposuit regemque&quot; SupplierCode=&quot;ra&quot; AllianceLevel=&quot;supplex aris imponet&quot; MembershipProgram=&quot;ipsa iovis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; CardNumber=&quot;maria ac&quot; Status=&quot;foedere certo&quot; MembershipStatus=&quot;sciret dare iussus&quot; FreeText=&quot;rapidi secum verrantque&quot; SupplierType=&quot;Air&quot; Level=&quot;omnipotens speluncis&quot; PriorityCode=&quot;premere et laxas&quot; VendorLocationRef=&quot;talia flammato&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;true&quot; ReservationLevel=&quot;true&quot;>
                  &lt;com:OperatedBy>et carcere&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;dare iussus&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
            &lt;com:DiscountCard Key=&quot;vasto rex aeolus&quot; Code=&quot;arce sce&quot; Description=&quot;luctantis ventos&quot; Number=&quot;bella gero&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
            &lt;com:SSR Key=&quot;imposuit regemque&quot; SegmentRef=&quot;ignem disiecitque rates&quot; PassiveSegmentRef=&quot;et premere&quot; ProviderReservationInfoRef=&quot;murmure montis&quot; Type=&quot;prem&quot; Status=&quot;rapidi secum&quot; FreeText=&quot;rapidum iaculata&quot; Carrier=&quot;pr&quot; CarrierSpecificText=&quot;insuper altos&quot; Description=&quot;maria ac terras&quot; ProviderDefinedType=&quot;rates evertitque&quot; SSRRuleRef=&quot;iovis rapidum&quot; URL=&quot;http://www.my.gov/ventis/sonoras&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; ProfileID=&quot;auras sed&quot; ProfileSecureFlightDocKey=&quot;iovisque et&quot;/>
            &lt;com:NameRemark Key=&quot;flammato secum dea&quot; Category=&quot;hic vasto rex&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:RemarkData>altos imposuit regemque&lt;/com:RemarkData>
               &lt;com:ProviderReservationInfoRef Key=&quot;profundum quippe&quot;/>
            &lt;/com:NameRemark>
            &lt;com:AirSeatAssignment Key=&quot;premit ac vinclis&quot; Status=&quot;lu&quot; Seat=&quot;luctantis ventos tempestatesque&quot; SeatTypeCode=&quot;nubi&quot; SegmentRef=&quot;et soror&quot; FlightDetailsRef=&quot;annos bella gero&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot; RailCoachNumber=&quot;iovisque et soror&quot;/>
            &lt;com:RailSeatAssignment Key=&quot;molemque et montis&quot; Status=&quot;mo&quot; Seat=&quot;profundum quippe&quot; RailSegmentRef=&quot;bella gero&quot; CoachNumber=&quot;incedo regina iovisque&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
               &lt;com:Characteristic SeatType=&quot;et premere et&quot; SeatDescription=&quot;faciat maria&quot; SeatValue=&quot;transfixo pectore flammas&quot; SeatValueDescription=&quot;claustra fremunt&quot;/>
            &lt;/com:RailSeatAssignment>
            &lt;com:EmergencyInfo>una cum&lt;/com:EmergencyInfo>
            &lt;com:Address Key=&quot;corde volutans nimborum&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
               &lt;com:AddressName>quae divum&lt;/com:AddressName>
               &lt;com:Street>caelumque profundum quippe&lt;/com:Street>
               &lt;com:City>sceptra tenens mollitque&lt;/com:City>
               &lt;com:State>volutans nimborum in&lt;/com:State>
               &lt;com:PostalCode>venit hic&lt;/com:PostalCode>
               &lt;com:Country>et&lt;/com:Country>
               &lt;com:ProviderReservationInfoRef Key=&quot;ferant rapidi&quot;/>
            &lt;/com:Address>
            &lt;com:DriversLicense Key=&quot;arce sceptra tenens&quot; LicenseNumber=&quot;ignem disiecitque rates&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;/>
            &lt;com:AppliedProfile Key=&quot;sedet aeolus&quot; TravelerID=&quot;scopuloque infixit acuto&quot; TravelerName=&quot;mollitque animos&quot; AccountID=&quot;bella gero&quot; AccountName=&quot;ast ego&quot; ImmediateParentID=&quot;nubibus ignem&quot; ImmediateParentName=&quot;aut supplex&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;/>
            &lt;com:CustomizedNameData Key=&quot;dare iussus habenas&quot; ProviderReservationInfoRef=&quot;imposuit regemque dedit&quot;>claustra fremunt celsa&lt;/com:CustomizedNameData>
            &lt;com:TravelComplianceData Key=&quot;pater omnipotens&quot; AirSegmentRef=&quot;quippe ferant rapidi&quot; PassiveSegmentRef=&quot;regina iovisque&quot; RailSegmentRef=&quot;exspirantem transfixo&quot; ReservationLocatorRef=&quot;honorem &quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
               &lt;com:PolicyCompliance InPolicy=&quot;false&quot; PolicyToken=&quot;in patriam loca&quot;/>
               &lt;com:ContractCompliance InContract=&quot;true&quot; ContractToken=&quot;quippe ferant&quot;/>
               &lt;com:PreferredSupplier Preferred=&quot;true&quot; ProfileType=&quot;Account&quot;/>
            &lt;/com:TravelComplianceData>
            &lt;com:TravelInfo TripName=&quot;illi indignantes&quot; TravelPurpose=&quot;ventos tempestatesque sonoras&quot;/>
         &lt;/com:BookingTraveler>
         &lt;com:OSI Key=&quot;molemque et montis&quot; Carrier=&quot;mo&quot; Code=&quot;tene&quot; Text=&quot;circum claustra&quot; ProviderReservationInfoRef=&quot;circum claustra fremunt&quot; ProviderCode=&quot;acuto&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
         &lt;com:AccountingRemark Key=&quot;ac terras caelumque&quot; Category=&quot;premere et lax&quot; TypeInGds=&quot;flammas turbine corripuit&quot; ProviderReservationInfoRef=&quot;ipsa iovis&quot; ProviderCode=&quot;sedet&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>vinclis et&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>in patriam loca&lt;/com:BookingTravelerRef>
         &lt;/com:AccountingRemark>
         &lt;com:GeneralRemark Key=&quot;talia flammato&quot; Category=&quot;auras sed&quot; TypeInGds=&quot;ventis illum exspirantem&quot; SupplierType=&quot;Rail&quot; ProviderReservationInfoRef=&quot;flammato secum&quot; ProviderCode=&quot;haben&quot; SupplierCode=&quot;caelu&quot; Direction=&quot;Incoming&quot; CreateDate=&quot;2000-01-16T23:10:25&quot; UseProviderNativeMode=&quot;false&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>gente tot&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>speluncis abdidit&lt;/com:BookingTravelerRef>
         &lt;/com:GeneralRemark>
         &lt;com:XMLRemark Key=&quot;patriam loca feta&quot; Category=&quot;molemque e&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>animos et&lt;/com:XMLRemark>
         &lt;com:UnassociatedRemark ProviderReservationInfoRef=&quot;montis circum&quot; ProviderCode=&quot;vento&quot; Key=&quot;et temperat&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:RemarkData>verrantque per auras&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>hic vasto rex&lt;/com:BookingTravelerRef>
         &lt;/com:UnassociatedRemark>
         &lt;com:Postscript ProviderReservationInfoRef=&quot;per auras sed&quot; ProviderCode=&quot;vincl&quot; Key=&quot;montis insuper altos&quot;>acuto ast ego&lt;/com:Postscript>
         &lt;com:PassiveInfo ProviderCode=&quot;secum dea&quot; ProviderLocatorCode=&quot;iras ni&quot; SupplierCode=&quot;et quisquam&quot; SupplierLocatorCode=&quot;carcere frenat illi&quot;>
            &lt;com:TicketNumber>iras ni&lt;/com:TicketNumber>
            &lt;com:ConfirmationNumber>iras ni&lt;/com:ConfirmationNumber>
            &lt;com:Commission Key=&quot;tempestatesque sonoras imperio&quot; Level=&quot;Fare&quot; Type=&quot;Flat&quot; Modifier=&quot;FarePercent&quot; Amount=&quot;sciret dare&quot; Value=&quot;montis circum&quot; Percentage=&quot;aeoliam venit hic&quot; BookingTravelerRef=&quot;talia flammato&quot; CommissionOverride=&quot;false&quot;/>
         &lt;/com:PassiveInfo>
         &lt;com:ContinuityCheckOverride Key=&quot;caelumque profundum&quot;>aeoliam venit&lt;/com:ContinuityCheckOverride>
         &lt;com:AgencyContactInfo Key=&quot;nubibus ignem&quot;>
            &lt;com:PhoneNumber Key=&quot;rapidi secum&quot; Type=&quot;Hotel&quot; Location=&quot;nimborum i&quot; CountryCode=&quot;caelu&quot; AreaCode=&quot;patriam lo&quot; Number=&quot;insuper altos&quot; Extension=&quot;et carcere&quot; Text=&quot;laxas sciret&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
               &lt;com:ProviderReservationInfoRef Key=&quot;abdidit atris&quot;/>
            &lt;/com:PhoneNumber>
         &lt;/com:AgencyContactInfo>
         &lt;com:CustomerID ProviderReservationInfoRef=&quot;magno cum murmure&quot; ProviderCode=&quot;annos&quot; Key=&quot;soror et&quot;>annos bella&lt;/com:CustomerID>
         &lt;com:FileFinishingInfo>
            &lt;com:ShopInformation CabinShopped=&quot;qui foedere&quot; CabinSelected=&quot;gero et&quot; LowestFareOffered=&quot;speluncis abdidit atris&quot;>
               &lt;com:SearchRequest Origin=&quot;per&quot; Destination=&quot;alt&quot; DepartureTime=&quot;aeolus arce sceptra&quot; ClassOfService=&quot;ve&quot;/>
               &lt;com:FlightsOffered Origin=&quot;alt&quot; Destination=&quot;ven&quot; DepartureTime=&quot;iussus habenas&quot; TravelOrder=&quot;3&quot; Carrier=&quot;ae&quot; FlightNumber=&quot;ignem&quot; ClassOfService=&quot;ge&quot; StopOver=&quot;false&quot; Connection=&quot;false&quot;/>
            &lt;/com:ShopInformation>
            &lt;com:PolicyInformation Type=&quot;Ticketing&quot; Name=&quot;anySimpleType&quot; OutOfPolicy=&quot;false&quot; SegmentRef=&quot;scopuloque infixit&quot;>
               &lt;com:ReasonCode>
                  &lt;com:OutOfPolicy>altos imposuit&lt;/com:OutOfPolicy>
                  &lt;com:PurposeOfTrip>scopuloque infixit&lt;/com:PurposeOfTrip>
                  &lt;com:Remark Key=&quot;ni faciat maria&quot;>regemque dedit qui&lt;/com:Remark>
               &lt;/com:ReasonCode>
            &lt;/com:PolicyInformation>
            &lt;com:AccountInformation AccountName=&quot;speluncis abdidit&quot;>
               &lt;com:Address Key=&quot;aeolus arce sceptra&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>ni faciat&lt;/com:AddressName>
                  &lt;com:Street>flammato secum dea&lt;/com:Street>
                  &lt;com:City>faciat maria&lt;/com:City>
                  &lt;com:State>ignem disiecitque&lt;/com:State>
                  &lt;com:PostalCode>illum exspirant&lt;/com:PostalCode>
                  &lt;com:Country>et&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;volutans nimborum&quot;/>
               &lt;/com:Address>
               &lt;com:PhoneNumber Key=&quot;feta furentibus&quot; Type=&quot;Fax&quot; Location=&quot;ni faciat &quot; CountryCode=&quot;impos&quot; AreaCode=&quot;laxas scir&quot; Number=&quot;maria ac terras&quot; Extension=&quot;furentibus&quot; Text=&quot;corde volutans&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;molemque et&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AccountInformation>
            &lt;com:AgencyInformation>
               &lt;com:Address Key=&quot;aut supplex&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:AddressName>flammato secum&lt;/com:AddressName>
                  &lt;com:Street>sceptra tenens mollitque&lt;/com:Street>
                  &lt;com:City>fremunt celsa&lt;/com:City>
                  &lt;com:State>aut supplex&lt;/com:State>
                  &lt;com:PostalCode>et montis insup&lt;/com:PostalCode>
                  &lt;com:Country>se&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;iras ni&quot;/>
               &lt;/com:Address>
               &lt;com:Email Key=&quot;coniunx una cum&quot; Type=&quot;quae divum&quot; Comment=&quot;circum claustra fremunt&quot; EmailID=&quot;et montis&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;rapidi secum&quot;/>
               &lt;/com:Email>
               &lt;com:PhoneNumber Key=&quot;disiecitque rates&quot; Type=&quot;Fax&quot; Location=&quot;molemque e&quot; CountryCode=&quot;patri&quot; AreaCode=&quot;celsa sede&quot; Number=&quot;exspirantem transfixo pectore&quot; Extension=&quot;coniunx un&quot; Text=&quot;aequora ventis illum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;supplex aris&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AgencyInformation>
            &lt;com:TravelerInformation HomeAirport=&quot;atr&quot; VisaExpirationDate=&quot;2003-07-02+05:30&quot; BookingTravelerRef=&quot;terras caelumque&quot;>
               &lt;com:EmergencyContact Name=&quot;anySimpleType&quot; Relationship=&quot;anySimpleType&quot;>
                  &lt;com:PhoneNumber Key=&quot;et premere et&quot; Type=&quot;Home&quot; Location=&quot;rex aeolus&quot; CountryCode=&quot;et mo&quot; AreaCode=&quot;imperio pr&quot; Number=&quot;secum dea&quot; Extension=&quot;regemque d&quot; Text=&quot;corde volutans&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;circum claustra fremunt&quot;/>
                  &lt;/com:PhoneNumber>
               &lt;/com:EmergencyContact>
            &lt;/com:TravelerInformation>
            &lt;com:CustomProfileInformation/>
         &lt;/com:FileFinishingInfo>
         &lt;com:CommissionRemark Key=&quot;circum claustra fremunt&quot; ProviderReservationInfoRef=&quot;pectore flammas&quot; ProviderCode=&quot;quae &quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:ProviderReservationLevel Amount=&quot;circum claustra&quot; Percentage=&quot;imperio premit&quot; CommissionCap=&quot;quae divum&quot;/>
            &lt;com:PassengerTypeLevel TravelerType=&quot;ni fa&quot; Amount=&quot;caelumque profundum quippe&quot; Percentage=&quot;aeolus arce&quot; CommissionCap=&quot;illi indignantes&quot;/>
         &lt;/com:CommissionRemark>
         &lt;com:ConsolidatorRemark Key=&quot;ac vinclis&quot; ProviderReservationInfoRef=&quot;habenas ipsa&quot; ProviderCode=&quot;sed p&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
            &lt;com:PseudoCityCode>ni faciat&lt;/com:PseudoCityCode>
         &lt;/com:ConsolidatorRemark>
         &lt;com:InvoiceRemark ProviderReservationInfoRef=&quot;iussus habenas&quot; ProviderCode=&quot;ipsa &quot; Key=&quot;dedit qui foedere&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
            &lt;com:RemarkData>patriam loca&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>auras sed&lt;/com:BookingTravelerRef>
            &lt;com:AirSegmentRef Key=&quot;tenens mollitque&quot;/>
            &lt;com:HotelReservationRef LocatorCode=&quot;exspiran&quot;/>
            &lt;com:VehicleReservationRef LocatorCode=&quot;exspiran&quot;/>
            &lt;com:PassiveSegmentRef Key=&quot;vinclis et&quot;/>
         &lt;/com:InvoiceRemark>
         &lt;com:SSR Key=&quot;divum incedo regina&quot; SegmentRef=&quot;furentibus austris&quot; PassiveSegmentRef=&quot;volutans nimborum&quot; ProviderReservationInfoRef=&quot;claustra fremunt celsa&quot; Type=&quot;et l&quot; Status=&quot;habenas ipsa&quot; FreeText=&quot;gente tot annos&quot; Carrier=&quot;pr&quot; CarrierSpecificText=&quot;et montis&quot; Description=&quot;ego quae&quot; ProviderDefinedType=&quot;dea corde voluta&quot; SSRRuleRef=&quot;celsa sedet&quot; URL=&quot;http://www.corp.com/flammato/et&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot; ProfileID=&quot;vasto rex&quot; ProfileSecureFlightDocKey=&quot;rapidum iaculata e&quot;/>
         &lt;com:EmailNotification Recipients=&quot;Default&quot;>
            &lt;com:EmailRef>infixit acuto ast&lt;/com:EmailRef>
         &lt;/com:EmailNotification>
         &lt;com:QueuePlace>
            &lt;com:PseudoCityCode>ipsa iovis&lt;/com:PseudoCityCode>
            &lt;com:QueueSelector Queue=&quot;corde volutans&quot; Category=&quot;iaculata e&quot; DateRange=&quot;carcere frenat illi&quot;/>
         &lt;/com:QueuePlace>
         &lt;com:SupplierLocator SupplierCode=&quot;au&quot; SupplierLocatorCode=&quot;molemque et&quot; ProviderReservationInfoRef=&quot;feta furentibus&quot; CreateDateTime=&quot;2009-06-19T08:21:06+05:30&quot;>
            &lt;com:SegmentRef Key=&quot;foedere certo&quot;/>
         &lt;/com:SupplierLocator>
         &lt;com:ThirdPartyInformation ThirdPartyCode=&quot;rapid&quot; ThirdPartyLocatorCode=&quot;quippe ferant&quot; ThirdPartyName=&quot;austris aeoliam&quot; ProviderReservationInfoRef=&quot;infixit acuto&quot; Key=&quot;animos et&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
            &lt;com:SegmentRef Key=&quot;et temperat&quot;/>
         &lt;/com:ThirdPartyInformation>
         &lt;pas:PassiveSegment SupplierCode=&quot;pater omnipotens&quot; Status=&quot;supplex aris imponet&quot; StartDate=&quot;iovis rapidum&quot; EndDate=&quot;et soror&quot; Origin=&quot;pre&quot; Destination=&quot;abd&quot; AvailabilitySource=&quot;quippe ferant rapidi&quot; PolledAvailabilityOption=&quot;caelumque profundum&quot; AvailabilityDisplayType=&quot;temperat iras&quot; FlightNumber=&quot;e nubibus&quot; ClassOfService=&quot;aequora ventis&quot; NumberOfItems=&quot;100&quot; SegmentType=&quot;transfixo pectore&quot; Key=&quot;iovis rapidum iaculata&quot; VehicleType=&quot;venit hic&quot; PassiveProviderReservationInfoRef=&quot;flammas turbine corripuit&quot; Group=&quot;100&quot; TravelOrder=&quot;100&quot; ProviderSegmentOrder=&quot;100&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot; PassiveGroup=&quot;turbine corripuit&quot;>
            &lt;pas:Amount Type=&quot;Text&quot; AmountDuePaid=&quot;evertitque aequora&quot;/>
         &lt;/pas:PassiveSegment>
         &lt;pas:PassiveRemark PassiveSegmentRef=&quot;illi indignantes&quot;>
            &lt;pas:Text>terras caelumque&lt;/pas:Text>
            &lt;pas:Type>rates evertitque&lt;/pas:Type>
         &lt;/pas:PassiveRemark>
         &lt;pas:AssociatedRemark ProviderReservationInfoRef=&quot;ni faciat maria&quot; ProviderCode=&quot;circu&quot; Key=&quot;aeolus antro luctantis&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot; SegmentRef=&quot;certo et&quot;>
            &lt;com:RemarkData>antro luctantis&lt;/com:RemarkData>
            &lt;com:BookingTravelerRef>terras caelumque&lt;/com:BookingTravelerRef>
         &lt;/pas:AssociatedRemark>
         &lt;com:ActionStatus Type=&quot;CXL&quot; TicketDate=&quot;metuens molemque et&quot; Key=&quot;talia flammato&quot; ProviderReservationInfoRef=&quot;hoc metuens&quot; QueueCategory=&quot;venit hic vasto&quot; AirportCode=&quot;son&quot; ProviderCode=&quot;iunon&quot; SupplierCode=&quot;ignem&quot; PseudoCityCode=&quot;tenens mol&quot; AccountCode=&quot;acuto ast&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
            &lt;com:Remark Key=&quot;montis insuper altos&quot;>quae divum incedo&lt;/com:Remark>
         &lt;/com:ActionStatus>
         &lt;com:ReviewBooking Key=&quot;talia flammato&quot; Queue=&quot;99&quot; QueueCategory=&quot;ae&quot; DateTime=&quot;2005-02-09T08:00:21&quot; PseudoCityCode=&quot;et coniunx&quot; ProviderCode=&quot;insup&quot; ProviderReservationInfoRef=&quot;annos bella gero&quot; Remarks=&quot;rapidum iaculata e&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;/>
      &lt;/univ:PassiveCreateReservationReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/PassiveService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
