<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>889d1f10-f53f-452c-8064-11ec22c95c51</elementGuidId>
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
      &lt;air:EMDIssuanceReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; UniversalRecordLocatorCode=&quot;regemque&quot; ShowDetails=&quot;false&quot; IssueAllOpenSVC=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;fremunt celsa&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;feran&quot; ProviderCode=&quot;scept&quot; AgentID=&quot;turbine corripuit scopuloque&quot;/>
         &lt;com:TerminalSessionInfo>flammato secum dea&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;profu&quot; PseudoCityCode=&quot;nubibus ig&quot;/>
         &lt;com:ProviderReservationDetail ProviderCode=&quot;ac vi&quot; ProviderLocatorCode=&quot;rapidum iaculat&quot; SupplierCode=&quot;caelu&quot;/>
         &lt;com:TicketNumber>circum claust&lt;/com:TicketNumber>
         &lt;air:IssuanceModifiers PlatingCarrier=&quot;co&quot;>
            &lt;com:FormOfPaymentRef Key=&quot;arce sceptra&quot;/>
            &lt;com:FormOfPayment Key=&quot;circum claustra fremunt&quot; Type=&quot;certo et premere&quot; FulfillmentType=&quot;et montis&quot; FulfillmentLocation=&quot;feta furentibus&quot; FulfillmentIDType=&quot;Bahn Card&quot; FulfillmentIDNumber=&quot;pectore flammas&quot; IsAgentType=&quot;false&quot; AgentText=&quot;pectore flammas&quot; ReuseFOP=&quot;annos bella gero&quot; ExternalReference=&quot;certo et&quot; Reusable=&quot;false&quot; ProfileID=&quot;rates evertitque aequora&quot; ProfileKey=&quot;volutans nimborum in&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
               &lt;com:CreditCard Type=&quot;et&quot; Number=&quot;et sororet soror&quot; ExpDate=&quot;2014-12&quot; Name=&quot;tenens mollitque animos&quot; CVV=&quot;mont&quot; ApprovalCode=&quot;habenas ipsa&quot; ExtendedPayment=&quot;mollitque animos&quot; CustomerReference=&quot;iovisque et&quot; AcceptanceOverride=&quot;true&quot; ThirdPartyPayment=&quot;false&quot; BankName=&quot;abdidit atris hoc&quot; BankCountryCode=&quot;co&quot; BankStateCode=&quot;ipsa i&quot; Enett=&quot;false&quot; ProfileID=&quot;frenat illi indignantes&quot; Key=&quot;nimborum in&quot;>
                  &lt;com:PhoneNumber Key=&quot;mollitque animos&quot; Type=&quot;Fax&quot; Location=&quot;insuper al&quot; CountryCode=&quot;feran&quot; AreaCode=&quot;nimborum i&quot; Number=&quot;vasto rex&quot; Extension=&quot;flammato s&quot; Text=&quot;tempestatesque sonoras&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;premit ac&quot;/>
                  &lt;/com:PhoneNumber>
                  &lt;com:BillingAddress Key=&quot;iras ni&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:AddressName>austris aeoliam venit&lt;/com:AddressName>
                     &lt;com:Street>qui foedere&lt;/com:Street>
                     &lt;com:City>atris hoc metuens&lt;/com:City>
                     &lt;com:State>atris hoc&lt;/com:State>
                     &lt;com:PostalCode>animos et tempe&lt;/com:PostalCode>
                     &lt;com:Country>so&lt;/com:Country>
                     &lt;com:ProviderReservationInfoRef Key=&quot;patriam loca&quot;/>
                  &lt;/com:BillingAddress>
               &lt;/com:CreditCard>
               &lt;com:DebitCard Type=&quot;mo&quot; Number=&quot;laxas sciret dare&quot; ExpDate=&quot;2010-06+05:30&quot; Name=&quot;vasto rex&quot; CVV=&quot;coni&quot; ApprovalCode=&quot;rapidum iaculata&quot; IssueNumber=&quot;turbine &quot; ProfileID=&quot;auras sed&quot; Key=&quot;aequora ventis illum&quot;>
                  &lt;com:PhoneNumber Key=&quot;aequora ventis&quot; Type=&quot;Mobile&quot; Location=&quot;aris impon&quot; CountryCode=&quot;imper&quot; AreaCode=&quot;arce scept&quot; Number=&quot;ni faciat&quot; Extension=&quot;circum cla&quot; Text=&quot;corde volutans&quot; ElStat=&quot;M&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;temperat iras&quot;/>
                  &lt;/com:PhoneNumber>
                  &lt;com:BillingAddress Key=&quot;magno cum&quot; ElStat=&quot;A&quot; KeyOverride=&quot;false&quot;>
                     &lt;com:AddressName>iunonis adorat praeterea&lt;/com:AddressName>
                     &lt;com:Street>premere et&lt;/com:Street>
                     &lt;com:City>iovisque et&lt;/com:City>
                     &lt;com:State>nubibus ignem disiecitque&lt;/com:State>
                     &lt;com:PostalCode>imposuit regemq&lt;/com:PostalCode>
                     &lt;com:Country>ra&lt;/com:Country>
                     &lt;com:ProviderReservationInfoRef Key=&quot;supplex aris imponet&quot;/>
                  &lt;/com:BillingAddress>
               &lt;/com:DebitCard>
               &lt;com:EnettVan MinPercentage=&quot;100&quot; MaxPercentage=&quot;100&quot; ExpiryDays=&quot;P1D&quot; MultiUse=&quot;true&quot;/>
               &lt;com:Certificate Number=&quot;volutans nimborum in&quot; Amount=&quot;maria ac&quot; DiscountAmount=&quot;foedere certo&quot; DiscountPercentage=&quot;100&quot; NotValidBefore=&quot;2018-03-16&quot; NotValidAfter=&quot;2012-03-31&quot;/>
               &lt;com:TicketNumber>et laxas scir&lt;/com:TicketNumber>
               &lt;com:Check MICRNumber=&quot;premit ac&quot; RoutingNumber=&quot;numen iunonis&quot; AccountNumber=&quot;vasto rex&quot; CheckNumber=&quot;profundum quippe&quot;/>
               &lt;com:Requisition Number=&quot;premit ac&quot; Category=&quot;Other&quot; Type=&quot;Credit&quot;/>
               &lt;com:MiscFormOfPayment CreditCardType=&quot;de&quot; CreditCardNumber=&quot;rapidum iaculata&quot; ExpDate=&quot;2003-03+05:30&quot; Text=&quot;imposuit regemque&quot; Category=&quot;ignem disiecitque rates&quot; AcceptanceOverride=&quot;true&quot;/>
               &lt;com:AgencyPayment AgencyBillingIdentifier=&quot;vinclis et carcere&quot; AgencyBillingNumber=&quot;aeolus antro&quot; AgencyBillingPassword=&quot;sonoras imperio&quot;/>
               &lt;com:UnitedNations Number=&quot;soror et&quot;/>
               &lt;com:DirectPayment Text=&quot;imperio premit ac&quot;/>
               &lt;com:AgentVoucher Number=&quot;transfixo pectore flammas&quot;/>
               &lt;com:PaymentAdvice Type=&quot;ego&quot; DocumentNumber=&quot;cum murmure&quot; IssueDate=&quot;2001-10-10+05:30&quot; IssueCity=&quot;ind&quot; OriginalFOP=&quot;sonoras imperio&quot;/>
               &lt;com:ProviderReservationInfoRef Key=&quot;rapidum iaculata e&quot; ProviderReservationLevel=&quot;true&quot;/>
               &lt;com:SegmentRef Key=&quot;auras sed&quot;/>
               &lt;com:BSPPayment BSPIdentifier=&quot;iovisque et&quot; BSPPassword=&quot;flammato secum dea&quot;/>
               &lt;com:ARCPayment ARCIdentifier=&quot;hic vasto rex&quot; ARCPassword=&quot;aeoliam venit&quot;/>
            &lt;/com:FormOfPayment>
            &lt;air:CustomerReceiptInfo BookingTravelerRef=&quot;altos imposuit regemque&quot; EmailRef=&quot;profundum quippe&quot;/>
            &lt;air:EMDEndorsement>premit ac vinclis&lt;/air:EMDEndorsement>
            &lt;air:EMDCommission Type=&quot;Amount&quot; Value=&quot;1000.00&quot; CurrencyCode=&quot;dar&quot;/>
         &lt;/air:IssuanceModifiers>
         &lt;air:SelectionModifiers SupplierCode=&quot;te&quot; RFIC=&quot;a&quot;>
            &lt;air:AirSegmentRef Key=&quot;sonoras imperio premit&quot;/>
            &lt;air:SvcSegmentRef>tot annos&lt;/air:SvcSegmentRef>
         &lt;/air:SelectionModifiers>
      &lt;/air:EMDIssuanceReq>
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
