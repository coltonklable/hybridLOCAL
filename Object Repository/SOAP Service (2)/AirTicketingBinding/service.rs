<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>8465010f-62d1-47d1-9fe2-b9afa5cd88c4</elementGuidId>
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
      &lt;air:AirTicketingReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; ReturnInfoOnFail=&quot;true&quot; BulkTicket=&quot;false&quot; ValidateSpanishResidency=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;air:AirReservationLocatorCode>nubibus&lt;/air:AirReservationLocatorCode>
         &lt;air:AirPricingInfoRef Key=&quot;ac vinclis et&quot;>
            &lt;com:BookingTravelerRef Key=&quot;rapidum iaculata e&quot;>
               &lt;com:LoyaltyCardRef Key=&quot;caelumque profundum&quot;/>
               &lt;com:DriversLicenseRef Key=&quot;circum claustra&quot;/>
               &lt;com:DiscountCardRef Key=&quot;coniunx una&quot;/>
               &lt;com:PaymentRef Key=&quot;arce sceptra&quot;/>
            &lt;/com:BookingTravelerRef>
         &lt;/air:AirPricingInfoRef>
         &lt;air:TicketingModifiersRef Key=&quot;circum claustra fremunt&quot;>
            &lt;air:AirPricingInfoRef Key=&quot;certo et premere&quot;/>
         &lt;/air:TicketingModifiersRef>
         &lt;air:WaiverCode TourCode=&quot;et montis&quot; TicketDesignator=&quot;feta furentibus&quot; Endorsement=&quot;sed pater&quot;/>
         &lt;com:Commission Key=&quot;talia flammato secum&quot; Level=&quot;Recalled&quot; Type=&quot;PercentTotal&quot; Modifier=&quot;FareAmount&quot; Amount=&quot;certo et&quot; Value=&quot;rates evertitqu&quot; Percentage=&quot;volutans nimborum in&quot; BookingTravelerRef=&quot;faciat maria&quot; CommissionOverride=&quot;false&quot;/>
         &lt;air:DetailedBillingInformation>
            &lt;com:FormOfPaymentRef Key=&quot;et quisquam&quot;/>
            &lt;air:AirPricingInfoRef Key=&quot;et soror&quot;/>
            &lt;air:BillingDetailItem Name=&quot;Destination&quot; DataType=&quot;Numeric&quot; MinLength=&quot;ventos tempestatesque sonoras&quot; MaxLength=&quot;regina iovisque&quot; Value=&quot;rapidum iaculata&quot;/>
         &lt;/air:DetailedBillingInformation>
         &lt;air:FaxDetailsInformation>
            &lt;air:AirPricingInfoRef Key=&quot;insuper altos&quot;/>
            &lt;air:FaxDetails IncludeCoverSheet=&quot;true&quot; To=&quot;iovisque et&quot; From=&quot;soror et coniunx&quot; DeptBillingCode=&quot;iunonis adorat&quot; InvoiceNumber=&quot;adorat praeterea&quot;>
               &lt;com:PhoneNumber Key=&quot;illum exspirantem transfixo&quot; Type=&quot;Hotel&quot; Location=&quot;nimborum i&quot; CountryCode=&quot;molli&quot; AreaCode=&quot;montis ins&quot; Number=&quot;molemque et montis&quot; Extension=&quot;gero et&quot; Text=&quot;circum claustra fremunt&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;aris imponet&quot;/>
               &lt;/com:PhoneNumber>
               &lt;air:TermConditions IncludeTermConditions=&quot;true&quot;>
                  &lt;air:LanguageOption Language=&quot;te&quot; Country=&quot;pr&quot;/>
               &lt;/air:TermConditions>
               &lt;com:Remark Key=&quot;iras ni&quot;>nubibus ignem&lt;/com:Remark>
            &lt;/air:FaxDetails>
         &lt;/air:FaxDetailsInformation>
         &lt;air:AirTicketingModifiers PlatingCarrier=&quot;au&quot; TicketedFareOverride=&quot;false&quot; SuppressTaxAndFee=&quot;false&quot; NoComparisonSFQ=&quot;false&quot;>
            &lt;air:DocumentModifiers GenerateItineraryInvoice=&quot;false&quot; GenerateAccountingInterface=&quot;false&quot;/>
            &lt;air:AirPricingInfoRef Key=&quot;qui foedere&quot;/>
            &lt;air:TourCode Value=&quot;atris hoc metue&quot;/>
            &lt;air:TicketEndorsement Value=&quot;atris hoc&quot;/>
            &lt;com:Commission Key=&quot;animos et temperat&quot; Level=&quot;Fare&quot; Type=&quot;Flat&quot; Modifier=&quot;SupplementaryPercent&quot; Amount=&quot;metuens molemque&quot; Value=&quot;dare iussus hab&quot; Percentage=&quot;tempestatesque sonoras imperio&quot; BookingTravelerRef=&quot;sceptra tenens mollitque&quot; CommissionOverride=&quot;false&quot;/>
            &lt;com:FormOfPayment Key=&quot;vasto rex&quot; Type=&quot;coniunx una cum&quot; FulfillmentType=&quot;rapidum iaculata&quot; FulfillmentLocation=&quot;turbine corripuit&quot; FulfillmentIDType=&quot;Bahn Card&quot; FulfillmentIDNumber=&quot;sciret dare&quot; IsAgentType=&quot;false&quot; AgentText=&quot;luctantis ventos&quot; ReuseFOP=&quot;ipsa iovis&quot; ExternalReference=&quot;aris imponet&quot; Reusable=&quot;false&quot; ProfileID=&quot;imperio premit&quot; ProfileKey=&quot;arce sceptra tenens&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
               &lt;com:CreditCard Type=&quot;ci&quot; Number=&quot;corde volutans&quot; ExpDate=&quot;2017-07&quot; Name=&quot;rapidum iaculata e&quot; CVV=&quot;nimb&quot; ApprovalCode=&quot;altos imposuit r&quot; ExtendedPayment=&quot;aeoliam venit&quot; CustomerReference=&quot;hic vasto&quot; AcceptanceOverride=&quot;false&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;nubibus ignem disiecitque&quot; BankCountryCode=&quot;im&quot; BankStateCode=&quot;rates &quot; Enett=&quot;false&quot; ProfileID=&quot;supplex aris imponet&quot; Key=&quot;ipsa iovis&quot;>
                  &lt;com:PhoneNumber Key=&quot;volutans nimborum in&quot; Type=&quot;Reservations&quot; Location=&quot;et carcere&quot; CountryCode=&quot;tempe&quot; AreaCode=&quot;ego quae&quot; Number=&quot;iaculata e nubibus&quot; Extension=&quot;omnipotens&quot; Text=&quot;premere et laxas&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;terras caelumque&quot;/>
                  &lt;/com:PhoneNumber>
                  &lt;com:BillingAddress Key=&quot;et carcere&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:AddressName>vasto rex aeolus&lt;/com:AddressName>
                     &lt;com:Street>arce sceptra&lt;/com:Street>
                     &lt;com:City>luctantis ventos&lt;/com:City>
                     &lt;com:State>bella gero&lt;/com:State>
                     &lt;com:PostalCode>ac vinclis et&lt;/com:PostalCode>
                     &lt;com:Country>im&lt;/com:Country>
                     &lt;com:ProviderReservationInfoRef Key=&quot;ignem disiecitque rates&quot;/>
                  &lt;/com:BillingAddress>
               &lt;/com:CreditCard>
               &lt;com:DebitCard Type=&quot;et&quot; Number=&quot;murmure montis&quot; ExpDate=&quot;2016-05&quot; Name=&quot;soror et&quot; CVV=&quot;impe&quot; ApprovalCode=&quot;transfixo pector&quot; IssueNumber=&quot;ego quae&quot; ProfileID=&quot;cum murmure&quot; Key=&quot;arce sceptra&quot;>
                  &lt;com:PhoneNumber Key=&quot;austris aeoliam venit&quot; Type=&quot;Reservations&quot; Location=&quot;sonoras im&quot; CountryCode=&quot;rapid&quot; AreaCode=&quot;auras sed&quot; Number=&quot;iovisque et&quot; Extension=&quot;flammato s&quot; Text=&quot;hic vasto rex&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;altos imposuit regemque&quot;/>
                  &lt;/com:PhoneNumber>
                  &lt;com:BillingAddress Key=&quot;profundum quippe&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:AddressName>luctantis ventos&lt;/com:AddressName>
                     &lt;com:Street>luctantis ventos tempestatesque&lt;/com:Street>
                     &lt;com:City>nubibus ignem&lt;/com:City>
                     &lt;com:State>et soror&lt;/com:State>
                     &lt;com:PostalCode>annos bella ger&lt;/com:PostalCode>
                     &lt;com:Country>ig&lt;/com:Country>
                     &lt;com:ProviderReservationInfoRef Key=&quot;iovisque et soror&quot;/>
                  &lt;/com:BillingAddress>
               &lt;/com:DebitCard>
               &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
               &lt;com:Certificate Number=&quot;montis insuper&quot; Amount=&quot;profundum quippe&quot; DiscountAmount=&quot;bella gero&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2009-10-30&quot; NotValidAfter=&quot;2001-03-09+05:30&quot;/>
               &lt;com:TicketNumber>faciat maria&lt;/com:TicketNumber>
               &lt;com:Check MICRNumber=&quot;transfixo pectore flammas&quot; RoutingNumber=&quot;claustra fremunt&quot; AccountNumber=&quot;una cum&quot; CheckNumber=&quot;corde volutans nimborum&quot;/>
               &lt;com:Requisition Number=&quot;et soror et&quot; Category=&quot;Government&quot; Type=&quot;Cash&quot;/>
               &lt;com:MiscFormOfPayment CreditCardType=&quot;ca&quot; CreditCardNumber=&quot;sceptra tenens mollitque&quot; ExpDate=&quot;2004-07+05:30&quot; Text=&quot;aut supplex aris&quot; Category=&quot;regemque dedit&quot; AcceptanceOverride=&quot;false&quot;/>
               &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;arce sceptra tenens&quot; AgencyBillingNumber=&quot;ignem disiecitque rates&quot; AgencyBillingPassword=&quot;pater omnipotens&quot;/>
               &lt;com:UnitedNations Number=&quot;sedet aeolus&quot;/>
               &lt;com:DirectPayment Text=&quot;scopuloque infixit acuto&quot;/>
               &lt;com:AgentVoucher Number=&quot;mollitque animos&quot;/>
               &lt;com:PaymentAdvice Type=&quot;bel&quot; DocumentNumber=&quot;ast ego&quot; IssueDate=&quot;2009-12-12+05:30&quot; IssueCity=&quot;ins&quot; OriginalFOP=&quot;imperio premit&quot;/>
               &lt;com:ProviderReservationInfoRef Key=&quot;hic vasto rex&quot; ProviderReservationLevel=&quot;true&quot;/>
               &lt;com:SegmentRef Key=&quot;murmure montis&quot;/>
               &lt;com:BSPPayment BSPIdentifier=&quot;illi indignantes magno&quot; BSPPassword=&quot;tot annos bella&quot;/>
               &lt;com:ARCPayment ARCIdentifier=&quot;profundum quippe&quot; ARCPassword=&quot;feta furentibus austris&quot;/>
            &lt;/com:FormOfPayment>
            &lt;com:CreditCardAuth Key=&quot;murmure montis&quot; PaymentRef=&quot;numen iunonis adorat&quot; TransId=&quot;ventos tempestatesque sonoras&quot; Number=&quot;in patriam loca&quot; Amount=&quot;caelumque profundum quippe&quot; AuthCode=&quot;sedet aeolus&quot; AuthResultCode=&quot;et temperat iras&quot; AVSResultCode=&quot;ventos tempestatesque&quot; Message=&quot;iunonis adorat&quot; ProviderReservationInfoRef=&quot;celsa sedet&quot; FormOfPaymentRef=&quot;scopuloque infixit&quot;/>
            &lt;com:Payment Key=&quot;ventos tempestatesque&quot; Type=&quot;DeliveryFee&quot; FormOfPaymentRef=&quot;circum claustra fremunt&quot; BookingTravelerRef=&quot;acuto ast&quot; Amount=&quot;honorem talia flammato&quot; AmountType=&quot;ac terras caelumque&quot; ApproximateAmount=&quot;premere et laxas&quot; Status=&quot;flammas turbine corripuit&quot; ElStat=&quot;M&quot; KeyOverride=&quot;true&quot;/>
         &lt;/air:AirTicketingModifiers>
         &lt;air:AirSegmentTicketingModifiers AirSegmentRef=&quot;sedet aeolus&quot; BrandTier=&quot;frenat ill&quot;/>
      &lt;/air:AirTicketingReq>
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
