<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>9a300d96-b0a4-489e-aafe-822d6ff63564</elementGuidId>
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
      &lt;air:AirExchangeEligibilityReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; Type=&quot;regemque dedit&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;fremunt celsa&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;feran&quot; ProviderCode=&quot;scept&quot; AgentID=&quot;turbine corripuit scopuloque&quot;/>
         &lt;com:TerminalSessionInfo>flammato secum dea&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;profu&quot; PseudoCityCode=&quot;nubibus ig&quot;/>
         &lt;air:ProviderReservationInfo ProviderCode=&quot;ac vi&quot; ProviderLocatorCode=&quot;rapidum iaculat&quot;/>
      &lt;/air:AirExchangeEligibilityReq>
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
