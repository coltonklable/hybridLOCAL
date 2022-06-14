<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>bfe7a501-70b5-4c87-a07a-073b89ff038c</elementGuidId>
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
         &lt;ses:SessTok id=&quot;cum murmure&quot;/>
         &lt;ses:SessProp nm=&quot;in patriam&quot; val=&quot;abdidit atris hoc&quot;/>
      &lt;/ses:SessionContext>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;air:AirFareRulesReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; FareRuleType=&quot;long&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;air:AirReservationSelector AirReservationLocatorCode=&quot;nubibus &quot;>
            &lt;air:FareInfoRef Key=&quot;ac vinclis et&quot;/>
         &lt;/air:AirReservationSelector>
         &lt;air:FareRuleLookup Origin=&quot;rap&quot; Destination=&quot;cae&quot; Carrier=&quot;ci&quot; FareBasis=&quot;coniunx una&quot; ProviderCode=&quot;arce &quot; DepartureDate=&quot;2015-11-05&quot; TicketingDate=&quot;2017-10-20&quot;>
            &lt;com:AccountCode Code=&quot;feta furentibus&quot; ProviderCode=&quot;sed p&quot; SupplierCode=&quot;talia&quot; Type=&quot;austris aeoliam&quot;/>
            &lt;com:PointOfSale ProviderCode=&quot;scire&quot; PseudoCityCode=&quot;dare iussu&quot; Key=&quot;bella gero&quot; IATA=&quot;dedit qu&quot;/>
         &lt;/air:FareRuleLookup>
         &lt;air:FareRuleKey FareInfoRef=&quot;magno cum&quot; ProviderCode=&quot;ac te&quot;>regemque dedit&lt;/air:FareRuleKey>
         &lt;air:AirFareDisplayRuleKey ProviderCode=&quot;infix&quot;>tenens mollitque animos&lt;/air:AirFareDisplayRuleKey>
         &lt;air:AirFareRulesModifier>
            &lt;air:AirFareRuleCategory FareInfoRef=&quot;montis insuper&quot;>
               &lt;air:CategoryCode>SEA&lt;/air:CategoryCode>
            &lt;/air:AirFareRuleCategory>
         &lt;/air:AirFareRulesModifier>
         &lt;air:FareRulesFilterCategory FareInfoRef=&quot;insuper altos&quot;>
            &lt;air:CategoryCode>anyType&lt;/air:CategoryCode>
         &lt;/air:FareRulesFilterCategory>
      &lt;/air:AirFareRulesReq>
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
