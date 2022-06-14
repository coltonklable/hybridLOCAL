<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request 1</name>
   <tag></tag>
   <elementGuidId>ce50ab19-92bc-479d-8dfc-9b98e279e0f1</elementGuidId>
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
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitCruiseTransaction>
         &lt;!--Optional:-->
         &lt;web:Profile>?&lt;/web:Profile>
         &lt;!--Optional:-->
         &lt;web:CorrelationToken>?&lt;/web:CorrelationToken>
         &lt;!--Optional:-->
         &lt;web:Transactions>
            &lt;!--You may enter ANY elements at this point-->
         &lt;/web:Transactions>
      &lt;/web:SubmitCruiseTransaction>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://americas.copy-webservices.travelport.com/B2BGateway/service/XMLSelect</soapServiceEndpoint>
   <soapServiceFunction>SubmitCruiseTransaction</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/C:/Users/arpita.singh/Desktop/XML%20select.wsdl</wsdlAddress>
</WebServiceRequestEntity>
