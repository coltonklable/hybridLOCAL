<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>6bc0caf3-d8ac-431f-ae4c-221277f7a1d2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJMzEwMjg0MDM0Ni0wODU2MTdlYjoxMjM0NQ==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ses=&quot;http://www.travelport.com/soa/common/security/SessionContext_v1&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
   &lt;soapenv:Body>
      &lt;air:LowFareSearchReq TargetBranch=&quot;P7149101&quot; SolutionResult=&quot;true&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:CityOrAirport Code=&quot;${From}&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:CityOrAirport Code=&quot;${To}&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime>
               &lt;com:TimeRange LatestTime=&quot;${DDate}&quot; EarliestTime=&quot;${DDate1}&quot; />
            &lt;/air:SearchDepTime>
         &lt;/air:SearchAirLeg>
         &lt;air:SearchAirLeg>
            &lt;air:SearchOrigin>
               &lt;com:CityOrAirport Code=&quot;${To}&quot;/>
            &lt;/air:SearchOrigin>
            &lt;air:SearchDestination>
               &lt;com:CityOrAirport Code=&quot;${From}&quot;/>
            &lt;/air:SearchDestination>
            &lt;air:SearchDepTime>
               &lt;com:TimeRange LatestTime=&quot;${RDate}&quot;  EarliestTime=&quot;${RDate1}&quot; />
            &lt;/air:SearchDepTime>
         &lt;/air:SearchAirLeg>
         &lt;air:AirSearchModifiers>
            &lt;air:PreferredProviders>
               &lt;com:Provider Code=&quot;1G&quot;/>
            &lt;/air:PreferredProviders>
            &lt;!--&lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;PG&quot;/>
            &lt;/air:PermittedCarriers>-->
            &lt;air:FlightType MaxStops=&quot;3&quot; MaxConnections=&quot;3&quot;/>
         &lt;/air:AirSearchModifiers>
         &lt;com:SearchPassenger Code=&quot;ADT&quot;/>
         &lt;air:AirPricingModifiers FaresIndicator=&quot;PublicFaresOnly&quot; CurrencyType=&quot;JPY&quot;>
            &lt;air:ExemptTaxes>
               &lt;air:TaxCategory>OB&lt;/air:TaxCategory>
            &lt;/air:ExemptTaxes>
         &lt;/air:AirPricingModifiers>
         &lt;!--&lt;com:SearchPassenger Code=&quot;CHD&quot;/>-->
      &lt;/air:LowFareSearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-001.PP.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.LFSLDate</defaultValue>
      <description></description>
      <id>79374d39-59e9-43a6-9584-ae5d39549392</id>
      <masked>false</masked>
      <name>DDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LFSEDate</defaultValue>
      <description></description>
      <id>530e9fff-34c3-407d-bdcc-ce43a590b20e</id>
      <masked>false</masked>
      <name>DDate1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LFSLDate1</defaultValue>
      <description></description>
      <id>8f8195a8-c4db-430c-a440-5f6831d507a2</id>
      <masked>false</masked>
      <name>RDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LFSEDate1</defaultValue>
      <description></description>
      <id>734836b5-bcf2-4f1e-a052-a9b42bd1483c</id>
      <masked>false</masked>
      <name>RDate1</name>
   </variables>
   <variables>
      <defaultValue>'LHR'</defaultValue>
      <description></description>
      <id>6e67fad2-476c-4590-b88c-47bd951a7c45</id>
      <masked>false</masked>
      <name>From</name>
   </variables>
   <variables>
      <defaultValue>'FRA'</defaultValue>
      <description></description>
      <id>64b0a4c3-bfe5-4c0e-b4e3-0f7627db31bc</id>
      <masked>false</masked>
      <name>To</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/air_v51_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
