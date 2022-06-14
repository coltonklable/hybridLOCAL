<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>c9e6dee5-ae54-47fe-ad7e-36626abe8f14</elementGuidId>
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
      &lt;univ:SupportedVersions urVersion=&quot;volutans nimborum in&quot; airVersion=&quot;maria ac&quot; hotelVersion=&quot;foedere certo&quot; vehicleVersion=&quot;sciret dare iussus&quot; passiveVersion=&quot;rapidi secum verrantque&quot; railVersion=&quot;montis circum&quot; cruiseVersion=&quot;et laxas sciret&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:PassiveCancelReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; PassiveReservationLocatorCode=&quot;regemque dedit&quot; ProviderCode=&quot;fremu&quot; ProviderLocatorCode=&quot;ferant rapidi&quot; Version=&quot;100&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;sceptra tenens&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;turbi&quot; ProviderCode=&quot;flamm&quot; AgentID=&quot;profundum quippe ferant&quot;/>
         &lt;com:TerminalSessionInfo>nubibus ignem disiecitque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;ac vi&quot; PseudoCityCode=&quot;rapidum ia&quot;/>
         &lt;pas:PassiveSegmentRef Key=&quot;caelumque profundum&quot;/>
         &lt;com:FileFinishingInfo>
            &lt;com:ShopInformation CabinShopped=&quot;circum claustra&quot; CabinSelected=&quot;coniunx una&quot; LowestFareOffered=&quot;arce sceptra&quot;>
               &lt;com:SearchRequest Origin=&quot;cir&quot; Destination=&quot;cer&quot; DepartureTime=&quot;et montis&quot; ClassOfService=&quot;fe&quot;/>
               &lt;com:FlightsOffered Origin=&quot;sed&quot; Destination=&quot;tal&quot; DepartureTime=&quot;austris aeoliam&quot; TravelOrder=&quot;3&quot; Carrier=&quot;sc&quot; FlightNumber=&quot;dare &quot; ClassOfService=&quot;be&quot; StopOver=&quot;false&quot; Connection=&quot;false&quot;/>
            &lt;/com:ShopInformation>
            &lt;com:PolicyInformation Type=&quot;Ticketing&quot; Name=&quot;anySimpleType&quot; OutOfPolicy=&quot;false&quot; SegmentRef=&quot;magno cum&quot;>
               &lt;com:ReasonCode>
                  &lt;com:OutOfPolicy>ac terras&lt;/com:OutOfPolicy>
                  &lt;com:PurposeOfTrip>regemque dedit&lt;/com:PurposeOfTrip>
                  &lt;com:Remark Key=&quot;infixit acuto&quot;>tenens mollitque animos&lt;/com:Remark>
               &lt;/com:ReasonCode>
            &lt;/com:PolicyInformation>
            &lt;com:AccountInformation AccountName=&quot;montis insuper&quot;>
               &lt;com:Address Key=&quot;habenas ipsa&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>iovisque et&lt;/com:AddressName>
                  &lt;com:Street>soror et coniunx&lt;/com:Street>
                  &lt;com:City>iunonis adorat&lt;/com:City>
                  &lt;com:State>adorat praeterea&lt;/com:State>
                  &lt;com:PostalCode>illum exspirant&lt;/com:PostalCode>
                  &lt;com:Country>ar&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;aris imponet&quot;/>
               &lt;/com:Address>
               &lt;com:PhoneNumber Key=&quot;premere et&quot; Type=&quot;Agency&quot; Location=&quot;molemque e&quot; CountryCode=&quot;gero &quot; AreaCode=&quot;circum cla&quot; Number=&quot;montis insuper&quot; Extension=&quot;aris impon&quot; Text=&quot;aris imponet&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;ac vinclis&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AccountInformation>
            &lt;com:AgencyInformation>
               &lt;com:Address Key=&quot;acuto ast&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:AddressName>dedit qui&lt;/com:AddressName>
                  &lt;com:Street>profundum quippe ferant&lt;/com:Street>
                  &lt;com:City>imposuit regemque&lt;/com:City>
                  &lt;com:State>magno cum&lt;/com:State>
                  &lt;com:PostalCode>iras ni faciat&lt;/com:PostalCode>
                  &lt;com:Country>ve&lt;/com:Country>
                  &lt;com:ProviderReservationInfoRef Key=&quot;metuens molemque&quot;/>
               &lt;/com:Address>
               &lt;com:Email Key=&quot;dare iussus habenas&quot; Type=&quot;tempestatesque sonoras imperio&quot; Comment=&quot;sceptra tenens mollitque&quot; EmailID=&quot;vasto rex&quot; ElStat=&quot;C&quot; KeyOverride=&quot;false&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;rapidum iaculata&quot;/>
               &lt;/com:Email>
               &lt;com:PhoneNumber Key=&quot;turbine corripuit&quot; Type=&quot;Business&quot; Location=&quot;sciret dar&quot; CountryCode=&quot;lucta&quot; AreaCode=&quot;ipsa iovis&quot; Number=&quot;aris imponet&quot; Extension=&quot;imperio pr&quot; Text=&quot;arce sceptra tenens&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>
                  &lt;com:ProviderReservationInfoRef Key=&quot;circum claustra fremunt&quot;/>
               &lt;/com:PhoneNumber>
            &lt;/com:AgencyInformation>
            &lt;com:TravelerInformation HomeAirport=&quot;cor&quot; VisaExpirationDate=&quot;2017-07-13&quot; BookingTravelerRef=&quot;rapidum iaculata e&quot;>
               &lt;com:EmergencyContact Name=&quot;anySimpleType&quot; Relationship=&quot;anySimpleType&quot;>
                  &lt;com:PhoneNumber Key=&quot;nimborum in&quot; Type=&quot;Email&quot; Location=&quot;iunonis ad&quot; CountryCode=&quot;preme&quot; AreaCode=&quot;iovisque e&quot; Number=&quot;nubibus ignem disiecitque&quot; Extension=&quot;imposuit r&quot; Text=&quot;rates evertitque&quot; ElStat=&quot;C&quot; KeyOverride=&quot;true&quot;>
                     &lt;com:ProviderReservationInfoRef Key=&quot;ipsa iovis&quot;/>
                  &lt;/com:PhoneNumber>
               &lt;/com:EmergencyContact>
            &lt;/com:TravelerInformation>
            &lt;com:CustomProfileInformation/>
         &lt;/com:FileFinishingInfo>
      &lt;/univ:PassiveCancelReq>
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
