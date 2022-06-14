<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>33fca7a4-e3d4-44a7-a2a2-923963c7905e</elementGuidId>
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
      &lt;air:AirRetrieveDocumentReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; ReturnRestrictions=&quot;true&quot; ReturnPricing=&quot;false&quot; RetrieveMCO=&quot;true&quot; UniversalRecordLocatorCode=&quot;foedere &quot; ProviderCode=&quot;profu&quot; ProviderLocatorCode=&quot;et carcere&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;iovis rapidum iaculata&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;spelu&quot; ProviderCode=&quot;bella&quot; AgentID=&quot;flammas turbine&quot;/>
         &lt;com:TerminalSessionInfo>hoc metuens&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;ac vi&quot; PseudoCityCode=&quot;speluncis &quot;/>
         &lt;air:AirReservationLocatorCode>aris imp&lt;/air:AirReservationLocatorCode>
         &lt;com:TicketNumber>praeterea aut&lt;/com:TicketNumber>
         &lt;air:TCRNumber>claustra fremunt&lt;/air:TCRNumber>
      &lt;/air:AirRetrieveDocumentReq>
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
